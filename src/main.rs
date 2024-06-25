use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read};
use std::process::{Command, exit};
use std::path::Path;
use std::env;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use toml;

#[derive(Serialize, Deserialize)]
struct Config {
    api: ApiConfig,
}

#[derive(Serialize, Deserialize)]
struct ApiConfig {
    token: String,
    url: String,
    model: String,
    max_tokens: i32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api: ApiConfig {
                token: String::new(),
                url: "https://api.openai.com/v1/chat/completions".to_string(),
                model: "gpt-4o".to_string(),
                max_tokens: 300,
            },
        }
    }
}

fn main() -> io::Result<()> {
    let config_path = dirs::home_dir().unwrap().join(".ponopush_config.toml");
    let prompt_path = Path::new("/etc/ponopush/ponopush.conf");
    let mut config: Config = if config_path.exists() {
        let mut config_content = String::new();
        let mut file = fs::File::open(&config_path)?;
        file.read_to_string(&mut config_content)?;
        toml::from_str(&config_content).unwrap_or_default()
    } else {
        Config::default()
    };

    if config.api.token.is_empty() {
        println!("OpenAI API Token not found.");
        print!("Please enter your OpenAI API Token: ");
        io::stdout().flush()?;
        let mut token = String::new();
        io::stdin().read_line(&mut token)?;
        config.api.token = token.trim().to_string();
        save_config(&config_path, &config)?;
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 4 && args[1] == "config" {
        match args[2].as_str() {
            "api.token" => config.api.token = args[3].clone(),
            "api.url" => config.api.url = args[3].clone(),
            "api.model" => config.api.model = args[3].clone(),
            "api.max_tokens" => config.api.max_tokens = args[3].parse().unwrap_or(config.api.max_tokens),
            _ => eprintln!("Invalid configuration key"),
        }
        save_config(&config_path, &config)?;
        return Ok(());
    }

    run_git_commands(&config, &prompt_path)?;

    Ok(())
}

fn save_config(path: &Path, config: &Config) -> io::Result<()> {
    let toml_string = toml::to_string(config).unwrap();
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}

async fn send_openai_request(api_url: &str, api_token: &str, model: &str, max_tokens: i32, prompt: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let body = serde_json::json!({
        "model": model,
        "messages": [{
            "role": "user",
            "content": prompt
        }],
        "max_tokens": max_tokens
    });

    let response = client.post(api_url)
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&body)
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;
    let response_text = response_json["choices"][0]["message"]["content"].as_str().unwrap_or("No response").to_string();
    Ok(response_text)
}

fn run_git_commands(config: &Config, prompt_path: &Path) -> io::Result<()> {
    let git_status = Command::new("git").arg("status").output()?;
    if !String::from_utf8_lossy(&git_status.stdout).contains(".gitignore") {
        Command::new("git").args(&["add", ".gitignore"]).output()?;
    }

    Command::new("git").args(&["add", "-A"]).output()?;
    let git_diff_output = Command::new("git").args(&["diff", "--cached"]).output()?;
    let diff = String::from_utf8_lossy(&git_diff_output.stdout);

    let mut prompt_template = String::new();
    let mut file = fs::File::open(prompt_path)?;
    file.read_to_string(&mut prompt_template)?;

    let prompt = format!("{}{}", prompt_template, diff);

    let commit_message = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(send_openai_request(&config.api.url, &config.api.token, &config.api.model, config.api.max_tokens, &prompt))
        .unwrap_or_else(|_| "Failed to get commit message from OpenAI".to_string());

    // Write commit message to a temporary file
    let commit_msg_file = "/tmp/COMMIT_EDITMSG";
    let mut file = fs::File::create(commit_msg_file)?;
    file.write_all(commit_message.as_bytes())?;
    file.sync_all()?;

    // Open the commit message file in the default editor
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let status = Command::new(editor)
        .arg(commit_msg_file)
        .status()
        .expect("Failed to open editor");

    if !status.success() {
        eprintln!("Editor exited with an error.");
        exit(1);
    }

    // Read the edited commit message
    let mut file = fs::File::open(commit_msg_file)?;
    let mut final_message = String::new();
    file.read_to_string(&mut final_message)?;

    if final_message.trim().is_empty() {
        eprintln!("Commit message cannot be empty.");
        exit(1);
    }

    let mut commit_file = fs::File::create(".git/COMMIT_EDITMSG")?;
    commit_file.write_all(final_message.as_bytes())?;

    Command::new("git").args(&["commit", "-F", ".git/COMMIT_EDITMSG"]).output()?;
    Command::new("git").arg("push").output()?;

    // Clean up
    fs::remove_file(commit_msg_file)?;

    Ok(())
}