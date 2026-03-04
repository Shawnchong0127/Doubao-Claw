use serde::{Deserialize, Serialize};

/// Well-known Doubao model identifiers.
pub struct MODELS;

impl MODELS {
    pub const PRO_32K:   &'static str = "doubao-pro-32k";
    pub const PRO_4K:    &'static str = "doubao-pro-4k";
    pub const LITE_32K:  &'static str = "doubao-lite-32k";
    pub const LITE_4K:   &'static str = "doubao-lite-4k";
    pub const PRO_128K:  &'static str = "doubao-pro-128k";

    pub fn all() -> Vec<ModelInfo> {
        vec![
            ModelInfo { id: Self::PRO_32K,  context: 32_768,  description: "Complex reasoning, long documents" },
            ModelInfo { id: Self::PRO_4K,   context: 4_096,   description: "Fast, everyday tasks" },
            ModelInfo { id: Self::LITE_32K, context: 32_768,  description: "Cost-efficient long context" },
            ModelInfo { id: Self::LITE_4K,  context: 4_096,   description: "Ultra-fast responses" },
            ModelInfo { id: Self::PRO_128K, context: 131_072, description: "Very long documents and codebases" },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub id:          &'static str,
    pub context:     u32,
    pub description: &'static str,
}

/// Per-request model configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model:       String,
    pub temperature: f32,
    pub max_tokens:  u32,
    pub top_p:       f32,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model:       MODELS::PRO_32K.to_string(),
            temperature: 0.7,
            max_tokens:  2048,
            top_p:       1.0,
        }
    }
}
