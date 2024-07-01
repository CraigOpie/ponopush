# PonoPush

PonoPush is a command-line application designed to assist developers in making better Git commit messages using AI. It leverages the OpenAI API to generate detailed commit messages based on the changes made to the codebase. PonoPush ensures that commit messages are informative, well-structured, and easy to understand.

## Features

- **Automatic Commit Message Generation**: Uses OpenAI API to create detailed and structured commit messages.
- **Customizable Prompts**: Users can customize the prompt used for generating commit messages.
- **Integration with Default Editor**: Opens the suggested commit message in the user's default editor for review and modification.
- **Configuration Management**: Allows setting API configurations such as `api.token`, `api.url`, `api.model`, and `api.max_tokens`.
- **GPG Signing**: Option to sign commits with GPG, controlled by the `gpg.enabled` configuration.
- **User-Provided Context**: Developers can provide additional context for the AI by using the `-m` flag.

## Installation

### Prerequisites

- Rust toolchain installed (use [rustup](https://rustup.rs/)).
- An OpenAI API token.

### Steps

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/ponopush.git
   cd ponopush
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. Install the binary:

   ```sh
   sudo cp target/release/ponopush /usr/local/bin/
   ```

4. Ensure the configuration directory and file exist:

   ```sh
   sudo mkdir -p /etc/ponopush
   sudo touch /etc/ponopush/ponopush.conf
   ```

5. Populate `/etc/ponopush/ponopush.conf` with your desired prompt template. Example:

   <!---
   ```plaintext
   The overall goal for the body of the commit message is to provide a detailed explanation of what the commit does, why the change was made, and any additional context that helps others understand the impact of the change. This can include:
   Reasoning: Explain why the change was necessary. This could involve describing a bug that was fixed, a feature that was added, or a refactor that was done. This should serve as an executive summary focusing on the reason for the changes.
   Details: Describe what was changed in more detail. Mention specific files or functions that were affected and any important considerations.
   Impact: Highlight the impact of the change. Mention if the change is backward-compatible, if it affects performance, or if it requires other changes elsewhere in the codebase.
   Related Issues: Reference any related issues, bug reports, or tickets that provide additional context for the change.

   Based on the goal of the body of the commit message, use the following git diff, generate a commit message adhering to these guidelines:
   1. The subject line should be short and to the point
   2. The subject line must be Title Case
   3. The subject line must use the imperative mood (be an order to do something)
   4. Don't put a period at the end of the Subject Line
   5. Place a blank line between the Subject Line and the Body
   6. The body should start with a detailed Reasoning section as an executive summary, followed by Details and Impact sections
   7. The body should be limited in line length to follow PEP 8 styling
   8. Describe what was done and why using bulleted formatting with the '-' character and indent as appropriate

   Example response:
   Refactor Commit View and Add New Files

   - Refactored 'CommitView.swift' for better layout and functionality to improve user experience and maintainability:
      - Replaced VStack with HStack for better alignment of elements
      - In the left pane, replaced 'Commit Changes' with 'Git Diff' for better context
      - Updated TextField styling for cleaner UI
      - Modified the alert background color for better visibility
      - The commit window size has been amended for better screen fit
      - The window's close functionality has been improved for better user experience

   - Created new files for more modular code:
      - 'Extensions.swift' for centralizing commonly used extensions
      - 'FileAccess.swift' for handling file access operations
      - 'OpenAPIService.swift' for interacting with the OpenAI API

   The first line should be the subject line, followed by a blank line, followed by the body content. Do not include "Subject:", "Body:", "Reasoning:", "Details:", "Impact:", nor "Related Issues:" in the response. Don't ever use ` nor " characters, instead use ' in your response. Your response must never be in a code block. Your response must never reference related issues or issue numbers.

   Git diff:
   ```
   --->

## Usage

### Setting Up Configuration

You can set the API configuration using the following commands:

```sh
ponopush config api.token YOUR_API_TOKEN
ponopush config api.url YOUR_API_URL
ponopush config api.model YOUR_API_MODEL
ponopush config api.max_tokens YOUR_API_MAX_TOKENS
ponopush config gpg.enabled true
```

### Running PonoPush

After making changes to your code, you can run:

```sh
ponopush
```

The application will generate a commit message, open it in your default editor for review, and commit the changes upon saving the file.

### Providing Additional Context

You can provide additional context for the AI by using the `-m` flag:

```sh
ponopush -m "User feedback demands that the users are allowed to provide a generic and non-professional reason on why the commit was made so that the AI can use the information to improve its response"
```

If the `-m` flag is not provided, the application will use the prompt from `/etc/ponopush/ponopush.conf` as usual.