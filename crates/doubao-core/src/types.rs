use serde::{Deserialize, Serialize};
use crate::models::ModelConfig;

// ── Roles ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

// ── Message ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role:    Role,
    pub content: String,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self { role: Role::System, content: content.into() }
    }
    pub fn user(content: impl Into<String>) -> Self {
        Self { role: Role::User, content: content.into() }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self { role: Role::Assistant, content: content.into() }
    }
}

// ── Request ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct ChatRequest {
    pub model:       String,
    pub messages:    Vec<Message>,
    pub temperature: f32,
    pub max_tokens:  u32,
    pub top_p:       f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream:      Option<bool>,
}

impl ChatRequest {
    pub fn new(config: ModelConfig, messages: Vec<Message>) -> Self {
        Self {
            model:       config.model,
            messages,
            temperature: config.temperature,
            max_tokens:  config.max_tokens,
            top_p:       config.top_p,
            stream:      None,
        }
    }

    pub fn streaming(mut self) -> Self {
        self.stream = Some(true);
        self
    }
}

// ── Non-streaming response ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id:      String,
    pub object:  String,
    pub created: u64,
    pub model:   String,
    pub choices: Vec<Choice>,
    pub usage:   Usage,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index:         u32,
    pub message:       Message,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens:     u32,
    pub completion_tokens: u32,
    pub total_tokens:      u32,
}

// ── Streaming chunk ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct StreamChunk {
    pub id:      String,
    pub object:  String,
    pub created: u64,
    pub model:   String,
    pub choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
pub struct StreamChoice {
    pub index:         u32,
    pub delta:         Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Delta {
    pub role:    Option<Role>,
    pub content: Option<String>,
}

// ── API error body ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ApiErrorBody {
    pub error: ApiErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorDetail {
    pub message: String,
    pub r#type:  Option<String>,
    pub code:    Option<String>,
}
