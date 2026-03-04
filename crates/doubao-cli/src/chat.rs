use anyhow::Result;
use colored::Colorize;
use doubao_api::DoubaoClient;
use doubao_core::{models::ModelConfig, types::ChatRequest, Message};
use futures::StreamExt;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::config::Config;

pub async fn interactive(client: DoubaoClient, cfg: &Config) -> Result<()> {
    println!("{}", "🐾 Doubao Claw — interactive chat".cyan().bold());
    println!(
        "{}",
        format!("   Model: {}  |  /help for commands  |  /exit to quit", cfg.model).dimmed()
    );
    println!();

    let mut rl = DefaultEditor::new()?;
    let mut history: Vec<Message> = Vec::new();

    loop {
        let prompt = format!("{} ", "you ▸".green().bold());
        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim().to_string();
                if input.is_empty() { continue; }
                let _ = rl.add_history_entry(&input);

                // Built-in commands
                match input.as_str() {
                    "/exit" | "/quit" | "/q" => {
                        println!("{}", "Bye! 🐾".cyan());
                        break;
                    }
                    "/clear" => {
                        history.clear();
                        println!("{}", "✓ Conversation cleared.".yellow());
                        continue;
                    }
                    "/help" => {
                        print_help();
                        continue;
                    }
                    "/history" => {
                        for (i, m) in history.iter().enumerate() {
                            let role = format!("{:?}", m.role).to_lowercase();
                            println!("[{}] {}: {}", i, role.cyan(), m.content);
                        }
                        continue;
                    }
                    _ => {}
                }

                history.push(Message::user(&input));

                let config = ModelConfig {
                    model:       cfg.model.clone(),
                    temperature: cfg.temperature,
                    max_tokens:  cfg.max_tokens,
                    top_p:       1.0,
                };
                let req = ChatRequest::new(config, history.clone());

                print!("{}", "🐾 doubao ▸ ".cyan().bold());

                let mut full_response = String::new();
                let mut stream = client.chat_stream(req);

                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(c) => {
                            if let Some(text) = &c.choices[0].delta.content {
                                print!("{text}");
                                full_response.push_str(text);
                                use std::io::Write;
                                let _ = std::io::stdout().flush();
                            }
                        }
                        Err(e) => {
                            eprintln!("\n{}", format!("Error: {e}").red());
                            break;
                        }
                    }
                }

                println!();
                if !full_response.is_empty() {
                    history.push(Message::assistant(full_response));
                }
            }

            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("{}", "\nBye! 🐾".cyan());
                break;
            }
            Err(e) => {
                eprintln!("{}", format!("Readline error: {e}").red());
                break;
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("{}", "── Commands ─────────────────────────────".cyan());
    println!("  /clear    — clear conversation history");
    println!("  /history  — show conversation history");
    println!("  /exit     — quit (also Ctrl+C / Ctrl+D)");
    println!("{}", "─────────────────────────────────────────".cyan());
}
