//! OpenCode Go provider
//!
//! OpenAI-compatible provider for OpenCode Go API.
//! Translates Anthropic API format to OpenAI format and back.
//! Model selection and routing are handled by jcode, not this provider.

use super::{EventStream, Provider};
use super::openai_request::build_tools;
use crate::message::{ContentBlock, Message, Role, StreamEvent, ToolDefinition};
use crate::provider_catalog::{
    load_api_key_from_env_or_config, openai_compatible_profile_is_configured,
    resolve_openai_compatible_profile, OPENCODE_GO_PROFILE,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

const DEFAULT_API_BASE: &str = "https://opencode.ai/zen/go/v1";
const DEFAULT_API_KEY_NAME: &str = "OPENCODE_GO_API_KEY";
const DEFAULT_ENV_FILE: &str = "opencode-go.env";
const DEFAULT_MODEL: &str = "THUDM/GLM-4.5";

/// Maximum number of retries for transient errors
const MAX_RETRIES: u32 = 3;

/// Base delay for exponential backoff (in milliseconds)
const RETRY_BASE_DELAY_MS: u64 = 1000;

pub struct OpenCodeGoProvider {
    client: Client,
    model: String,
    api_base: String,
    api_key: String,
}

impl OpenCodeGoProvider {
    /// Create a new OpenCode Go provider from the profile
    pub fn from_profile() -> Result<Self> {
        let profile = resolve_openai_compatible_profile(OPENCODE_GO_PROFILE);
        let api_key = load_api_key_from_env_or_config(&profile.api_key_env, &profile.env_file)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "{} not found. Set the environment variable or add to {}",
                    profile.api_key_env,
                    profile.env_file
                )
            })?;

        let model = profile
            .default_model
            .map(|s| s.to_string())
            .unwrap_or_else(|| DEFAULT_MODEL.to_string());

        Ok(Self {
            client: crate::provider::shared_http_client(),
            model,
            api_base: profile.api_base,
            api_key,
        })
    }

    /// Create a new OpenCode Go provider with explicit configuration
    pub fn new(api_base: String, api_key: String, model: String) -> Self {
        Self {
            client: crate::provider::shared_http_client(),
            model,
            api_base: normalize_api_base(&api_base).unwrap_or_else(|_| DEFAULT_API_BASE.to_string()),
            api_key,
        }
    }

    /// Check if OpenCode Go provider is configured
    pub fn configured() -> bool {
        openai_compatible_profile_is_configured(OPENCODE_GO_PROFILE)
    }

    async fn complete_openai(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
        system: &str,
    ) -> Result<EventStream> {
        let url = format!("{}/chat/completions", self.api_base);

        // Build OpenAI-compatible request
        let openai_messages = build_openai_messages(messages, system)?;
        let openai_tools = build_tools(tools);

        let mut request_body = serde_json::json!({
            "model": self.model,
            "messages": openai_messages,
            "stream": true,
        });

        if !openai_tools.is_empty() {
            request_body["tools"] = serde_json::to_value(&openai_tools)?;
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to OpenCode Go API")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = crate::util::http_error_body(response, "HTTP error").await;
            anyhow::bail!("OpenCode Go API error ({}): {}", status, body);
        }

        // Convert OpenAI streaming response to Anthropic format
        let stream = response.bytes_stream();
        let event_stream = convert_openai_stream_to_anthropic(stream).await?;
        Ok(event_stream)
    }
}

#[async_trait]
impl Provider for OpenCodeGoProvider {
    async fn complete(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
        system: &str,
        _resume_session_id: Option<&str>,
    ) -> Result<EventStream> {
        self.complete_openai(messages, tools, system).await
    }

    fn name(&self) -> &str {
        "opencode-go"
    }

    fn fork(&self) -> Arc<dyn Provider> {
        Arc::new(Self {
            client: self.client.clone(),
            model: self.model.clone(),
            api_base: self.api_base.clone(),
            api_key: self.api_key.clone(),
        })
    }

    fn available_models_display(&self) -> Vec<String> {
        vec![self.model.clone()]
    }

    async fn prefetch_models(&self) -> Result<()> {
        // Model catalog is handled by jcode, not this provider
        Ok(())
    }
}

/// Build OpenAI-compatible messages from jcode messages
fn build_openai_messages(messages: &[Message], system: &str) -> Result<Vec<Value>> {
    let mut openai_messages = Vec::new();

    // Add system message if present (as first user message since OpenAI doesn't have system role in chat completions)
    if !system.is_empty() {
        openai_messages.push(serde_json::json!({
            "role": "user",
            "content": system
        }));
    }

    // Convert jcode messages to OpenAI format
    for msg in messages {
        let role = match msg.role {
            Role::User => "user",
            Role::Assistant => "assistant",
        };

        let content: Value = if msg.content.len() == 1 {
            match &msg.content[0] {
                ContentBlock::Text { text, .. } => serde_json::json!(text),
                ContentBlock::ToolResult { tool_use_id, content, .. } => serde_json::json!({
                    "role": "tool",
                    "tool_call_id": tool_use_id,
                    "content": content
                }),
                ContentBlock::ToolUse { id, name, input } => serde_json::json!({
                    "role": "assistant",
                    "tool_calls": [{
                        "id": id,
                        "type": "function",
                        "function": {
                            "name": name,
                            "arguments": input
                        }
                    }]
                }),
                _ => serde_json::json!(format!("{:?}", msg.content[0])),
            }
        } else {
            // Multiple content blocks
            serde_json::json!(msg.content
                .iter()
                .map(|block| match block {
                    ContentBlock::Text { text, .. } => serde_json::json!({
                        "type": "text",
                        "text": text
                    }),
                    ContentBlock::ToolUse { id, name, input } => serde_json::json!({
                        "type": "tool_use",
                        "id": id,
                        "name": name,
                        "input": input
                    }),
                    _ => serde_json::json!(null),
                })
                .collect::<Vec<_>>())
        };

        openai_messages.push(serde_json::json!({
            "role": role,
            "content": content
        }));
    }

    Ok(openai_messages)
}

/// Convert OpenAI streaming response to Anthropic format
async fn convert_openai_stream_to_anthropic(
    stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Unpin + Send + 'static,
) -> Result<EventStream> {
    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut stream = stream;
        let mut buffer = String::new();

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    let chunk_str = String::from_utf8_lossy(&chunk);
                    buffer.push_str(&chunk_str);

                    // Process SSE events
                    for line in buffer.lines() {
                        if line.starts_with("data: ") {
                            let data = &line[6..];
                            if data == "[DONE]" {
                                let _ = tx.send(Ok(StreamEvent::MessageEnd { stop_reason: None })).await;
                                return;
                            }

                            if let Ok(json) = serde_json::from_str::<Value>(data) {
                                if let Some(choices) = json.get("choices").and_then(|v| v.as_array()) {
                                    if let Some(choice) = choices.first() {
                                        if let Some(delta) = choice.get("delta") {
                                            if let Some(content) = delta.get("content").and_then(|v| v.as_str()) {
                                                if !content.is_empty() {
                                                    let _ = tx.send(Ok(StreamEvent::TextDelta(content.to_string()))).await;
                                                }
                                            }
                                        }

                                        if let Some(finish_reason) = choice.get("finish_reason").and_then(|v| v.as_str()) {
                                            if finish_reason == "stop" {
                                                let _ = tx.send(Ok(StreamEvent::MessageEnd { stop_reason: Some(finish_reason.to_string()) })).await;
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Keep only incomplete lines in buffer
                    if let Some(last_newline) = buffer.rfind('\n') {
                        buffer = buffer[last_newline + 1..].to_string();
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(anyhow::anyhow!("Stream error: {}", e))).await;
                    return;
                }
            }
        }

        let _ = tx.send(Ok(StreamEvent::MessageEnd { stop_reason: None })).await;
    });

    Ok(Box::pin(ReceiverStream::new(rx)))
}

/// Normalize API base URL
fn normalize_api_base(raw: &str) -> Result<String> {
    let url = url::Url::parse(raw)
        .context("Invalid API base URL")?;

    let normalized = if url.path().ends_with("/v1") || url.path().ends_with("/v1/") {
        url.to_string()
    } else {
        let path = url.path().trim_end_matches('/');
        let base = url.path_segments()
            .map(|segments| {
                let base: Vec<&str> = segments.collect();
                if base.last().map_or(false, |s| *s == "v1") {
                    format!("{}/v1", base[..base.len()-1].join("/"))
                } else {
                    format!("{}/v1", base.join("/"))
                }
            })
            .unwrap_or_else(|| format!("{}/v1", path));

        url.join(&base)?
            .to_string()
    };

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_api_base() {
        assert_eq!(
            normalize_api_base("https://opencode.ai/zen/go/v1").unwrap(),
            "https://opencode.ai/zen/go/v1"
        );
        assert_eq!(
            normalize_api_base("https://opencode.ai/zen/go/v1/").unwrap(),
            "https://opencode.ai/zen/go/v1/"
        );
        assert_eq!(
            normalize_api_base("https://opencode.ai/zen/go").unwrap(),
            "https://opencode.ai/zen/go/v1"
        );
    }
}
