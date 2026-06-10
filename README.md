# ShellClaw

ShellClaw is a lightweight command-line tool written in Rust that translates natural language descriptions into terminal commands and executes them upon your approval. It automatically detects your operating system to target **PowerShell** (on Windows) or **Bash** (on Linux / macOS).

It is powered by Mistral AI (`mistral-tiny`).

## Features

- **Natural Language Translation**: Simply state what you want to achieve, and ShellClaw translates it to a shell command.
- **Safety Prompt**: Displays the proposed command and asks for confirmation before executing it.
- **Context-Aware System Instructions**: Adapts system prompt and target shell dynamically based on your OS.
- **Clean Execution Output**: Clearly demarcates command outputs and errors.

## Prerequisites

- **Rust & Cargo**: Make sure you have Rust installed. If not, get it from [rustup.rs](https://rustup.rs/).
- **Mistral AI API Key**: You need an active API key from Mistral AI.

## Installation & Setup

1. **Clone or download** this repository.
2. In the root directory, create a `.env` file containing your API key:
   ```env
   MISTRAL_API_KEY=your_actual_mistral_api_key_here
   ```
   *(Note: `.env` is automatically ignored by Git to keep your key secure.)*

## Usage

Run the program using Cargo:

```bash
cargo run
```

### Example Interaction

```text
ShellClaw - Natural Language to Command Line
==============================================

What do you want to do? > show all files in the current folder with their sizes

Command: Get-ChildItem | Select-Object Name, Length
Execute? [Y/n] > 

--- Output ---

Name               Length
----               ------
Cargo.lock          43137
Cargo.toml            261
src
target

--- End ---
```

To quit the program, type `exit`, `quit`, or leave the input empty.
