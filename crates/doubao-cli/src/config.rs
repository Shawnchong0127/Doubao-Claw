use anyhow::{Context, Result};
use colored::Colorize;
use doubao_api::DoubaoClient;
use doubao_core::DoubaoError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key:     Option<String>,
    pub model:       String,
    pub temperature: f32,
    pub max_tokens:  u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key:     None,
            model:       "doubao-pro-32k".to_string(),
            temperature: 0.7,
            max_tokens:  2048,
        }
    }
}

impl Config {
    fn config_path() -> Result<PathBuf> {
        let dir = dirs::config_dir()
            .context("Cannot determine config directory")?
            .join("doubao-claw");
        std::fs::create_dir_all(&dir)?;
        Ok(dir.join("config.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&raw).unwrap_or_default())
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        println!("{}", "✓ Config saved.".green());
        Ok(())
    }

    pub fn build_client(&self) -> Result<DoubaoClient, DoubaoError> {
        let key = self.api_key.clone().unwrap_or_default();
        DoubaoClient::new(key)
    }

    pub fn get_value(&self, key: &str) {
        match key {
            "api_key" => println!("{}", self.api_key.as_deref().unwrap_or("<not set>")),
            "model"   => println!("{}", self.model),
            "temperature" => println!("{}", self.temperature),
            "max_tokens"  => println!("{}", self.max_tokens),
            _         => eprintln!("{}", format!("Unknown key: {key}").red()),
        }
    }

    pub fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "api_key"     => self.api_key = Some(value.to_string()),
            "model"       => self.model = value.to_string(),
            "temperature" => self.temperature = value.parse().context("temperature must be a float")?,
            "max_tokens"  => self.max_tokens = value.parse().context("max_tokens must be an integer")?,
            _             => anyhow::bail!("Unknown config key: {key}"),
        }
        Ok(())
    }

    pub fn show(&self) {
        let masked = self.api_key.as_deref().map(|k| {
            if k.len() > 8 { format!("{}...{}", &k[..4], &k[k.len()-4..]) }
            else { "****".to_string() }
        }).unwrap_or_else(|| "<not set>".to_string());

        println!("{}", "── Doubao Claw Config ──────────────────".cyan());
        println!("  api_key     : {masked}");
        println!("  model       : {}", self.model);
        println!("  temperature : {}", self.temperature);
        println!("  max_tokens  : {}", self.max_tokens);
        println!("{}", "────────────────────────────────────────".cyan());
    }
}
