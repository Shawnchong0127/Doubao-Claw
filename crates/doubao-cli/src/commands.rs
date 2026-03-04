use anyhow::Result;
use colored::Colorize;
use doubao_api::DoubaoClient;
use doubao_core::{models::{ModelConfig, MODELS}, types::ChatRequest, Message};
use futures::StreamExt;

use crate::config::Config;

/// One-shot `ask` subcommand — streams to stdout.
pub async fn ask_once(client: DoubaoClient, cfg: &Config, prompt: &str) -> Result<()> {
    let config = ModelConfig {
        model:       cfg.model.clone(),
        temperature: cfg.temperature,
        max_tokens:  cfg.max_tokens,
        top_p:       1.0,
    };
    let req = ChatRequest::new(config, vec![Message::user(prompt)]);
    let mut stream = client.chat_stream(req);

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(c) => {
                if let Some(text) = &c.choices[0].delta.content {
                    print!("{text}");
                    use std::io::Write;
                    let _ = std::io::stdout().flush();
                }
                if c.choices[0].finish_reason.is_some() { break; }
            }
            Err(e) => {
                eprintln!("\n{}", format!("Error: {e}").red());
                std::process::exit(1);
            }
        }
    }

    println!();
    Ok(())
}

/// List all known models.
pub fn list_models() {
    println!("{}", "── Available Doubao Models ─────────────────────────────────".cyan());
    println!(
        "  {:<20} {:>10}  {}",
        "ID".bold(),
        "Context".bold(),
        "Description".bold()
    );
    println!("  {}", "─".repeat(55));
    for m in MODELS::all() {
        println!(
            "  {:<20} {:>8}K  {}",
            m.id.green(),
            m.context / 1024,
            m.description.dimmed(),
        );
    }
    println!("{}", "────────────────────────────────────────────────────────────".cyan());
    println!(
        "  {} Get your key at {}",
        "Tip:".yellow(),
        "https://console.volcengine.com/".underline()
    );
}
