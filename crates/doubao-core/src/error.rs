use thiserror::Error;

#[derive(Debug, Error)]
pub enum DoubaoError {
    #[error("API error {status}: {message}")]
    Api { status: u16, message: String },

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Missing API key — set DOUBAO_API_KEY or run `dbclaw config set api_key <key>`")]
    MissingApiKey,

    #[error("Stream ended unexpectedly")]
    StreamEnded,

    #[error("Config error: {0}")]
    Config(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

// Allow reqwest import for From impl
extern crate reqwest;
