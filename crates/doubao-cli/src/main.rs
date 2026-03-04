mod config;
mod chat;
mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use tracing_subscriber::{EnvFilter, fmt};

// ── CLI definition ─────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name    = "dbclaw",
    version = env!("CARGO_PKG_VERSION"),
    about   = "🐾 Doubao Claw — blazing-fast Doubao AI from your terminal",
    long_about = None,
)]
struct Cli {
    /// Doubao API key (overrides config and DOUBAO_API_KEY env var)
    #[arg(long, env = "DOUBAO_API_KEY", global = true, hide_env_values = true)]
    api_key: Option<String>,

    /// Model to use (e.g. doubao-pro-32k)
    #[arg(long, short = 'm', global = true)]
    model: Option<String>,

    /// Enable verbose logging
    #[arg(long, short = 'v', global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an interactive chat session
    Chat,

    /// Ask a single question and exit
    Ask {
        /// Your question
        prompt: Vec<String>,
    },

    /// List available Doubao models
    Models,

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Get a config value
    Get { key: String },
    /// Set a config value
    Set { key: String, value: String },
    /// Show all config values
    Show,
}

// ── Entry point ────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Logging
    let level = if cli.verbose { "debug" } else { "warn" };
    fmt()
        .with_env_filter(EnvFilter::new(level))
        .with_target(false)
        .init();

    // Load config, allow CLI flag to override
    let mut cfg = config::Config::load()?;
    if let Some(key) = cli.api_key {
        cfg.api_key = Some(key);
    }
    if let Some(model) = cli.model {
        cfg.model = model;
    }

    match cli.command {
        Commands::Chat => {
            let client = cfg.build_client()?;
            chat::interactive(client, &cfg).await?;
        }

        Commands::Ask { prompt } => {
            let text = prompt.join(" ");
            if text.is_empty() {
                eprintln!("{}", "Error: provide a prompt after `ask`".red());
                std::process::exit(1);
            }
            let client = cfg.build_client()?;
            commands::ask_once(client, &cfg, &text).await?;
        }

        Commands::Models => {
            commands::list_models();
        }

        Commands::Config { action } => match action {
            ConfigAction::Get { key }         => cfg.get_value(&key),
            ConfigAction::Set { key, value }  => { cfg.set_value(&key, &value)?; cfg.save()?; }
            ConfigAction::Show               => cfg.show(),
        },
    }

    Ok(())
}
