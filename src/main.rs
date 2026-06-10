use reqwest::Client;
use serde::Serialize;
use std::env;
use std::io::{self, Write};

#[derive(Serialize, Clone)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Clone)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const COLOR_YELLOW: &str = "\x1b[93m";
    const COLOR_DIM: &str = "\x1b[90m";
    const COLOR_RESET: &str = "\x1b[0m";

    #[cfg(target_os = "windows")]
    const SHELL_CMD: &str = "powershell";
    #[cfg(target_os = "windows")]
    const SHELL_ARGS: &[&str] = &["-Command"];
    #[cfg(target_os = "windows")]
    const SYSTEM_PROMPT: &str = "Du bist ein PowerShell-Assistent für Windows. Der Benutzer beschreibt in natürlicher Sprache, was er tun möchte. \
                                 Gib NUR den passenden PowerShell-Befehl zurück, ohne Erklärung, ohne Backticks, ohne Titel. \
                                 Wenn der Benutzer etwas Unsinniges oder Gefährliches fragt, antworte mit 'FEHLER: [Grund]'.";

    #[cfg(not(target_os = "windows"))]
    const SHELL_CMD: &str = "bash";
    #[cfg(not(target_os = "windows"))]
    const SHELL_ARGS: &[&str] = &["-c"];
    #[cfg(not(target_os = "windows"))]
    const SYSTEM_PROMPT: &str = "Du bist ein Bash-Assistent für Linux (Raspberry Pi). Der Benutzer beschreibt in natürlicher Sprache, was er tun möchte. \
                                 Gib NUR den passenden Bash-Befehl zurück, ohne Erklärung, ohne Backticks, ohne Titel. \
                                 Wenn der Benutzer etwas Unsinniges oder Gefährliches fragt, antworte mit 'FEHLER: [Grund]'.";

    println!("{}ShellClaw - Natural Language to Command Line", COLOR_DIM);
    println!("==============================================\n{}", COLOR_RESET);

    dotenv::dotenv().ok();

    let api_key = env::var("MISTRAL_API_KEY")
        .expect("Bitte MISTRAL_API_KEY in .env Datei setzen");

    let client = Client::new();

    let mut conversation_history = vec![
        ChatMessage {
            role: "system".to_string(),
            content: SYSTEM_PROMPT.to_string(),
        }
    ];

    loop {
        print!("{}Was möchtest du tun? > {}", COLOR_DIM, COLOR_RESET);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() || input == "exit" || input == "quit" {
            println!("{}Tschüss!{}", COLOR_DIM, COLOR_RESET);
            break;
        }

        conversation_history.push(ChatMessage {
            role: "user".to_string(),
            content: input.to_string(),
        });

        let request = ChatRequest {
            model: "mistral-tiny".to_string(),
            messages: conversation_history.clone(),
        };

        let response = client
            .post("https://api.mistral.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let text = response.text().await?;

        let result: serde_json::Value = serde_json::from_str(&text)?;

        if let Some(choice) = result["choices"].as_array().and_then(|a| a.first()) {
            let command = sanitize_command(choice["message"]["content"].as_str().unwrap_or(""));

            conversation_history.push(ChatMessage {
                role: "assistant".to_string(),
                content: command.clone(),
            });

            if command.starts_with("FEHLER:") {
                println!("\n{}{}{}", COLOR_DIM, command, COLOR_RESET);
            } else {
                println!("\n{}Befehl: {}{}{}", COLOR_DIM, COLOR_YELLOW, command, COLOR_RESET);
                print!("{}Ausführen? [J/n] > {}", COLOR_DIM, COLOR_RESET);
                io::stdout().flush()?;

                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm)?;
                let confirm = confirm.trim().to_lowercase();

                if confirm.is_empty() || confirm == "j" || confirm == "y" {
                    println!("\n{}--- Ausgabe ---\n{}", COLOR_DIM, COLOR_RESET);
                    let mut cmd = std::process::Command::new(SHELL_CMD);
                    for arg in SHELL_ARGS {
                        cmd.arg(arg);
                    }
                    let output = cmd.arg(&command).output();

                    match output {
                        Ok(out) => {
                            if !out.stdout.is_empty() {
                                print!("{}", String::from_utf8_lossy(&out.stdout));
                            }
                            if !out.stderr.is_empty() {
                                eprint!("{}", String::from_utf8_lossy(&out.stderr));
                            }
                        }
                        Err(e) => {
                            println!("{}Fehler beim Ausführen: {}{}", COLOR_DIM, e, COLOR_RESET);
                        }
                    }
                    println!("\n{}--- Ende ---{}", COLOR_DIM, COLOR_RESET);
                }
            }
        }

        println!();
    }

    Ok(())
}

fn sanitize_command(cmd: &str) -> String {
    let mut s = cmd.trim().to_string();

    // Strip markdown code block markers (fenced code blocks)
    if s.starts_with("```") {
        if let Some(end_idx) = s.rfind("```") {
            if end_idx > 3 {
                s = s[3..end_idx].to_string();
            }
        }
    }

    // Strip leading "powershell" if the code block had it (e.g. ```powershell)
    s = s.trim().to_string();
    if s.to_lowercase().starts_with("powershell") {
        s = s["powershell".len()..].to_string();
    }

    // Strip any remaining leading/trailing backticks
    s = s.trim().trim_matches('`').trim().to_string();

    s
}