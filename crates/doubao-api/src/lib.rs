use async_stream::try_stream;
use bytes::Bytes;
use doubao_core::{
    error::DoubaoError,
    types::{ApiErrorBody, ChatRequest, ChatResponse, ModelConfig, StreamChunk},
    Message,
};
use futures::Stream;
use reqwest::{header, Client};
use serde_json::Value;
use tracing::debug;

const API_BASE: &str = "https://ark.cn-beijing.volces.com/api/v3";

// ── Client ─────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct DoubaoClient {
    http:    Client,
    api_key: String,
    base:    String,
}

impl DoubaoClient {
    /// Create a new client. Reads `DOUBAO_API_KEY` from the environment if
    /// `api_key` is empty.
    pub fn new(api_key: impl Into<String>) -> Result<Self, DoubaoError> {
        let key = api_key.into();
        let key = if key.is_empty() {
            std::env::var("DOUBAO_API_KEY").map_err(|_| DoubaoError::MissingApiKey)?
        } else {
            key
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {key}"))
                .map_err(|e| DoubaoError::Config(e.to_string()))?,
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let http = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(DoubaoError::Http)?;

        Ok(Self { http, api_key: key, base: API_BASE.to_string() })
    }

    /// Override the base URL (useful for testing / proxies).
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base = url.into();
        self
    }

    // ── Non-streaming ──────────────────────────────────────────────────────

    pub async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, DoubaoError> {
        let url = format!("{}/chat/completions", self.base);
        debug!("POST {url}");

        let resp = self.http.post(&url).json(&req).send().await?;
        let status = resp.status().as_u16();

        if status >= 400 {
            let body: ApiErrorBody = resp.json().await?;
            return Err(DoubaoError::Api { status, message: body.error.message });
        }

        Ok(resp.json::<ChatResponse>().await?)
    }

    // ── Streaming ──────────────────────────────────────────────────────────

    pub fn chat_stream(
        &self,
        req: ChatRequest,
    ) -> impl Stream<Item = Result<StreamChunk, DoubaoError>> + '_ {
        let req = req.streaming();
        let url = format!("{}/chat/completions", self.base);

        try_stream! {
            debug!("POST {url} (stream)");
            let resp = self.http.post(&url).json(&req).send().await?;
            let status = resp.status().as_u16();

            if status >= 400 {
                let body: ApiErrorBody = resp.json().await?;
                Err(DoubaoError::Api { status, message: body.error.message })?;
                return;
            }

            let mut stream = resp.bytes_stream();
            use futures::StreamExt;
            let mut buf = String::new();

            while let Some(chunk) = stream.next().await {
                let bytes: Bytes = chunk?;
                buf.push_str(&String::from_utf8_lossy(&bytes));

                // SSE lines may arrive split across chunks
                while let Some(pos) = buf.find('\n') {
                    let line = buf[..pos].trim().to_string();
                    buf = buf[pos + 1..].to_string();

                    if line.is_empty() { continue; }
                    if line == "data: [DONE]" { return; }
                    if let Some(data) = line.strip_prefix("data: ") {
                        let chunk: StreamChunk = serde_json::from_str(data)?;
                        yield chunk;
                    }
                }
            }
        }
    }

    // ── Convenience helpers ────────────────────────────────────────────────

    /// Single-turn, non-streaming chat with default model config.
    pub async fn ask(
        &self,
        prompt: impl Into<String>,
    ) -> Result<String, DoubaoError> {
        let req = ChatRequest::new(
            ModelConfig::default(),
            vec![Message::user(prompt)],
        );
        let resp = self.chat(req).await?;
        Ok(resp.choices[0].message.content.clone())
    }

    /// Returns the API key (masked) for display purposes.
    pub fn masked_key(&self) -> String {
        let k = &self.api_key;
        if k.len() > 8 {
            format!("{}...{}", &k[..4], &k[k.len() - 4..])
        } else {
            "****".to_string()
        }
    }
}
