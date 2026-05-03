//! OpenCode Go provider
//!
//! OpenAI-compatible provider for OpenCode Go API.
//! Translates Anthropic API format to OpenAI format and back.
//! Model selection and routing are handled by jcode, not this provider.

use super::{EventStream, Provider};
use super::common::{recover_rwlock_read, recover_rwlock_write};
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
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

const DEFAULT_API_BASE: &str = "https://opencode.ai/zen/go/v1";
const DEFAULT_MODEL: &str = "deepseek-v4-flash";

const KNOWN_MODELS: &[&str] = &["deepseek-v4-flash", "THUDM/GLM-4.5"];

pub(crate) fn is_known_model(model: &str) -> bool {
    let normalized = model.trim();
    KNOWN_MODELS.contains(&normalized)
}

pub struct OpenCodeGoProvider {
    client: Client,
    model: Arc<RwLock<String>>,
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
            model: Arc::new(RwLock::new(model)),
            api_base: profile.api_base,
            api_key,
        })
    }

    /// Create a new OpenCode Go provider with explicit configuration
    pub fn new(api_base: String, api_key: String, model: String) -> Self {
        Self {
            client: crate::provider::shared_http_client(),
            model: Arc::new(RwLock::new(model)),
            api_base: crate::provider_catalog::normalize_api_base(&api_base).unwrap_or_else(|_| {
                crate::logging::warn(&format!("Failed to normalize API base '{}', using default: {}", api_base, DEFAULT_API_BASE));
                DEFAULT_API_BASE.to_string()
            }),
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

        let model = recover_rwlock_read(&self.model, |guard| guard.to_string(), "opencode-go", "model read");
        let mut request_body = serde_json::json!({
            "model": model,
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
            anyhow::bail!(
                "OpenCode Go API error ({}): {} (api_base={}, model={})",
                status,
                body,
                self.api_base,
                self.model()
            );
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
            model: Arc::clone(&self.model),
            api_base: self.api_base.clone(),
            api_key: self.api_key.clone(),
        })
    }

    fn model(&self) -> String {
        recover_rwlock_read(&self.model, |guard| guard.to_string(), "opencode-go", "model read")
    }

    fn set_model(&self, model: &str) -> anyhow::Result<()> {
        let trimmed = model.trim();
        if trimmed.is_empty() {
            anyhow::bail!("OpenCode Go model cannot be empty");
        }
        recover_rwlock_write(&self.model, |guard| *guard = trimmed.to_string(), "opencode-go", "model write");
        Ok(())
    }

    fn available_models_display(&self) -> Vec<String> {
        KNOWN_MODELS.iter().map(|s| s.to_string()).collect()
    }

    fn available_models_for_switching(&self) -> Vec<String> {
        KNOWN_MODELS.iter().map(|s| s.to_string()).collect()
    }

    fn context_window(&self) -> usize {
        // Context windows for OpenCode Go models (from official documentation)
        // deepseek-v4-flash: 1,048,576 tokens (1M) from OpenRouter/DeepSeek docs
        // THUDM/GLM-4.5: 131,072 tokens from Z.AI docs (GLM-4.5-Air)
        match self.model().as_str() {
            "deepseek-v4-flash" => 1_048_576,
            "THUDM/GLM-4.5" => 131_072,
            _ => 131_072, // Conservative default for unknown models
        }
    }

    async fn prefetch_models(&self) -> Result<()> {
        // Model catalog is handled by jcode, not this provider
        Ok(())
    }
}

/// Build OpenAI-compatible messages from jcode messages
fn build_openai_messages(messages: &[Message], system: &str) -> Result<Vec<Value>> {
    let mut openai_messages = Vec::new();

    // Add system message if present (OpenAI chat completions API supports system role)
    if !system.is_empty() {
        openai_messages.push(serde_json::json!({
            "role": "system",
            "content": system
        }));
    }

    // Convert jcode messages to OpenAI format
    for msg in messages {
        // Handle ToolResult blocks as separate messages (OpenAI format)
        for block in &msg.content {
            if let ContentBlock::ToolResult { tool_use_id, content, is_error } = block {
                let output = if is_error == &Some(true) {
                    format!("[Error] {}", content)
                } else {
                    content.clone()
                };
                openai_messages.push(serde_json::json!({
                    "role": "tool",
                    "tool_call_id": tool_use_id,
                    "content": output
                }));
            }
        }

        // Process non-ToolResult blocks as a single message
        let non_tool_blocks: Vec<_> = msg.content
            .iter()
            .filter(|block| !matches!(block, ContentBlock::ToolResult { .. }))
            .collect();

        if non_tool_blocks.is_empty() {
            continue; // Skip if only ToolResult blocks were present
        }

        let role = match msg.role {
            Role::User => "user",
            Role::Assistant => "assistant",
        };

        let content: Value = if non_tool_blocks.len() == 1 {
            match &non_tool_blocks[0] {
                ContentBlock::Text { text, .. } => serde_json::json!(text),
                ContentBlock::Reasoning { text } => serde_json::json!(text),
                ContentBlock::ToolUse { id, name, input } => {
                    let arguments = if input.is_object() {
                        match serde_json::to_string(input) {
                            Ok(s) => s,
                            Err(e) => {
                                crate::logging::warn(&format!("Failed to serialize tool input to JSON: {}", e));
                                "{}".to_string()
                            }
                        }
                    } else {
                        input.as_str().unwrap_or_else(|| {
                            crate::logging::warn("Tool input is not a string, using empty object");
                            "{}"
                        }).to_string()
                    };
                    serde_json::json!({
                        "role": "assistant",
                        "tool_calls": [{
                            "id": id,
                            "type": "function",
                            "function": {
                                "name": name,
                                "arguments": arguments
                            }
                        }]
                    })
                },
                _ => anyhow::bail!("Unsupported content block type in single-block message: {:?}", non_tool_blocks[0]),
            }
        } else {
            // Multiple content blocks
            let has_tool_use = non_tool_blocks.iter().any(|b| matches!(b, ContentBlock::ToolUse { .. }));
            if has_tool_use {
                // If there's a ToolUse in multi-block, we need to handle it specially
                let tool_calls: Vec<Value> = non_tool_blocks
                    .iter()
                    .filter_map(|block| {
                        if let ContentBlock::ToolUse { id, name, input } = block {
                            let arguments = if input.is_object() {
                                match serde_json::to_string(input) {
                                    Ok(s) => s,
                                    Err(e) => {
                                        crate::logging::warn(&format!("Failed to serialize tool input to JSON: {}", e));
                                        "{}".to_string()
                                    }
                                }
                            } else {
                                input.as_str().unwrap_or_else(|| {
                                    crate::logging::warn("Tool input is not a string, using empty object");
                                    "{}"
                                }).to_string()
                            };
                            Some(serde_json::json!({
                                "id": id,
                                "type": "function",
                                "function": {
                                    "name": name,
                                    "arguments": arguments
                                }
                            }))
                        } else {
                            None
                        }
                    })
                    .collect();

                let text_content: Vec<Value> = non_tool_blocks
                    .iter()
                    .filter_map(|block| match block {
                        ContentBlock::Text { text, .. } => Some(serde_json::json!(text)),
                        ContentBlock::Reasoning { text } => Some(serde_json::json!(text)),
                        _ => None,
                    })
                    .collect();

                if text_content.is_empty() {
                    serde_json::json!(tool_calls)
                } else {
                    serde_json::json!({
                        "content": text_content,
                        "tool_calls": tool_calls
                    })
                }
            } else {
                // Multiple non-tool blocks (text/reasoning)
                serde_json::json!(non_tool_blocks
                    .iter()
                    .map(|block| Ok(match block {
                        ContentBlock::Text { text, .. } => serde_json::json!({
                            "type": "text",
                            "text": text
                        }),
                        ContentBlock::Reasoning { text } => serde_json::json!({
                            "type": "text",
                            "text": text
                        }),
                        _ => anyhow::bail!("Unsupported content block type in multi-block message: {:?}", block),
                    }))
                    .collect::<Result<Vec<_>, _>>()?)
            }
        };

        // If content has tool_calls at top level, use that directly
        if let Some(tool_calls) = content.get("tool_calls") {
            openai_messages.push(serde_json::json!({
                "role": role,
                "tool_calls": tool_calls,
                "content": content.get("content").unwrap_or(&serde_json::json!(""))
            }));
        } else {
            openai_messages.push(serde_json::json!({
                "role": role,
                "content": content
            }));
        }
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
        const MAX_BUFFER_SIZE: usize = 1024 * 1024; // 1MB limit to prevent memory issues
        let mut message_end_sent = false;
        // Track partial tool calls for streaming argument accumulation
        let mut partial_tool_calls: std::collections::HashMap<String, (String, String)> = std::collections::HashMap::new();

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    let chunk_str = String::from_utf8_lossy(&chunk);
                    buffer.push_str(&chunk_str);

                    // Normalize line endings to \n for consistent processing
                    let normalized = buffer.replace("\r\n", "\n").replace('\r', "\n");

                    // Process SSE events
                    for line in normalized.lines() {
                        if line.starts_with("data: ") {
                            let data = &line[6..];
                            if data == "[DONE]" {
                                if !message_end_sent {
                                    if tx.send(Ok(StreamEvent::MessageEnd { stop_reason: None })).await.is_err() {
                                        return;
                                    }
                                    message_end_sent = true;
                                }
                                return;
                            }

                            if let Ok(json) = serde_json::from_str::<Value>(data) {
                                if let Some(choices) = json.get("choices").and_then(|v| v.as_array()) {
                                    if let Some(choice) = choices.first() {
                                        if let Some(delta) = choice.get("delta") {
                                            // Handle content deltas
                                            if let Some(content) = delta.get("content").and_then(|v| v.as_str()) {
                                                if !content.is_empty() {
                                                    if tx.send(Ok(StreamEvent::TextDelta(content.to_string()))).await.is_err() {
                                                        return;
                                                    }
                                                }
                                            }

                                            // Handle tool call deltas
                                            if let Some(tool_calls) = delta.get("tool_calls").and_then(|v| v.as_array()) {
                                                for tool_call in tool_calls {
                                                    if let Some(id) = tool_call.get("id").and_then(|v| v.as_str()) {
                                                        if let Some(function) = tool_call.get("function") {
                                                            let name = function.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                                            let arguments = function.get("arguments").and_then(|v| v.as_str()).unwrap_or("");

                                                            // Accumulate arguments for this tool call
                                                            partial_tool_calls.entry(id.to_string())
                                                                .and_modify(|(existing_name, existing_args)| {
                                                                    if !name.is_empty() {
                                                                        *existing_name = name.to_string();
                                                                    }
                                                                    existing_args.push_str(arguments);
                                                                })
                                                                .or_insert((name.to_string(), arguments.to_string()));
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        if let Some(finish_reason) = choice.get("finish_reason").and_then(|v| v.as_str()) {
                                            // Send accumulated tool calls when stream finishes
                                            if !partial_tool_calls.is_empty() {
                                                for (id, (name, arguments)) in partial_tool_calls.drain() {
                                                    if tx.send(Ok(StreamEvent::ToolUseStart { id, name })).await.is_err() {
                                                        return;
                                                    }
                                                    if !arguments.is_empty() {
                                                        if tx.send(Ok(StreamEvent::ToolInputDelta(arguments))).await.is_err() {
                                                            return;
                                                        }
                                                    }
                                                    if tx.send(Ok(StreamEvent::ToolUseEnd)).await.is_err() {
                                                        return;
                                                    }
                                                }
                                            }
                                            if finish_reason == "stop" {
                                                if !message_end_sent {
                                                    if tx.send(Ok(StreamEvent::MessageEnd { stop_reason: Some(finish_reason.to_string()) })).await.is_err() {
                                                        return;
                                                    }
                                                    message_end_sent = true;
                                                }
                                                return;
                                            }
                                        }
                                    }
                                }
                            } else {
                                // Log JSON parse error
                                crate::logging::warn(&format!("Failed to parse SSE JSON: {}", data));
                            }
                        }
                    }

                    // Keep only incomplete lines in buffer (use normalized buffer)
                    if let Some(last_newline) = normalized.rfind('\n') {
                        buffer = normalized[last_newline + 1..].to_string();
                    } else {
                        // No newline found - check buffer size to prevent unbounded growth
                        if normalized.len() > MAX_BUFFER_SIZE {
                            if tx.send(Err(anyhow::anyhow!("Stream buffer exceeded {} bytes, clearing to prevent memory issues", MAX_BUFFER_SIZE))).await.is_err() {
                                return;
                            }
                            buffer.clear();
                        } else {
                            buffer = normalized;
                        }
                    }
                }
                Err(e) => {
                    if tx.send(Err(anyhow::anyhow!("Stream error: {}", e))).await.is_err() {
                        return;
                    }
                    return;
                }
            }
        }

        // Send final MessageEnd if not already sent
        if !message_end_sent {
            if tx.send(Ok(StreamEvent::MessageEnd { stop_reason: None })).await.is_err() {
                return;
            }
        }
    });

    Ok(Box::pin(ReceiverStream::new(rx)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{ContentBlock, Message, Role};

    #[test]
    fn test_normalize_api_base() {
        assert_eq!(
            crate::provider_catalog::normalize_api_base("https://opencode.ai/zen/go/v1").unwrap(),
            "https://opencode.ai/zen/go/v1"
        );
        assert_eq!(
            crate::provider_catalog::normalize_api_base("https://opencode.ai/zen/go/v1/").unwrap(),
            "https://opencode.ai/zen/go/v1/"
        );
        assert_eq!(
            crate::provider_catalog::normalize_api_base("https://opencode.ai/zen/go").unwrap(),
            "https://opencode.ai/zen/go/v1"
        );
    }

    #[test]
    fn test_known_models() {
        assert!(is_known_model("deepseek-v4-flash"));
        assert!(is_known_model("THUDM/GLM-4.5"));
        assert!(!is_known_model("unknown-model"));
        assert!(is_known_model("  deepseek-v4-flash  ")); // trim handling
    }

    #[test]
    fn test_available_models_uses_constant() {
        let provider = OpenCodeGoProvider::new(
            "https://opencode.ai/zen/go/v1".to_string(),
            "test_key".to_string(),
            "deepseek-v4-flash".to_string(),
        );

        let display = provider.available_models_display();
        let switching = provider.available_models_for_switching();

        assert_eq!(display.len(), KNOWN_MODELS.len());
        assert_eq!(switching.len(), KNOWN_MODELS.len());
        assert!(display.contains(&"deepseek-v4-flash".to_string()));
        assert!(display.contains(&"THUDM/GLM-4.5".to_string()));
    }

    #[test]
    fn test_context_window() {
        let provider = OpenCodeGoProvider::new(
            "https://opencode.ai/zen/go/v1".to_string(),
            "test_key".to_string(),
            "deepseek-v4-flash".to_string(),
        );

        assert_eq!(provider.context_window(), 1_048_576);

        provider.set_model("THUDM/GLM-4.5").unwrap();
        assert_eq!(provider.context_window(), 131_072);

        provider.set_model("unknown-model").unwrap();
        assert_eq!(provider.context_window(), 131_072); // conservative default
    }

    #[test]
    fn test_set_model_empty() {
        let provider = OpenCodeGoProvider::new(
            "https://opencode.ai/zen/go/v1".to_string(),
            "test_key".to_string(),
            "deepseek-v4-flash".to_string(),
        );

        assert!(provider.set_model("").is_err());
        assert!(provider.set_model("  ").is_err());
        assert!(provider.set_model("valid-model").is_ok());
    }

    #[test]
    fn test_build_openai_messages_simple() {
        let messages = vec![
            Message {
                role: Role::User,
                content: vec![ContentBlock::Text { text: "Hello".to_string(), cache_control: None }],
            },
        ];

        let result = build_openai_messages(&messages, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["role"], "user");
        assert_eq!(result[0]["content"], "Hello");
    }

    #[test]
    fn test_build_openai_messages_with_system() {
        let messages = vec![
            Message {
                role: Role::User,
                content: vec![ContentBlock::Text { text: "Hello".to_string(), cache_control: None }],
            },
        ];

        let result = build_openai_messages(&messages, "You are a helpful assistant").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0]["role"], "system");
        assert_eq!(result[1]["role"], "user");
    }

    #[test]
    fn test_build_openai_messages_tool_use() {
        let messages = vec![
            Message {
                role: Role::Assistant,
                content: vec![
                    ContentBlock::ToolUse {
                        id: "call_123".to_string(),
                        name: "test_tool".to_string(),
                        input: serde_json::json!({"param": "value"}),
                    },
                ],
            },
        ];

        let result = build_openai_messages(&messages, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["role"], "assistant");
        assert!(result[0].get("tool_calls").is_some());
    }

    #[test]
    fn test_build_openai_messages_tool_result() {
        let messages = vec![
            Message {
                role: Role::Assistant,
                content: vec![ContentBlock::ToolUse {
                    id: "call_123".to_string(),
                    name: "test_tool".to_string(),
                    input: serde_json::json!({"param": "value"}),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentBlock::ToolResult {
                    tool_use_id: "call_123".to_string(),
                    content: "Result".to_string(),
                    is_error: Some(false),
                }],
            },
        ];

        let result = build_openai_messages(&messages, "").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[1]["role"], "tool");
        assert_eq!(result[1]["tool_call_id"], "call_123");
    }

    #[test]
    fn test_provider_fork() {
        let provider = OpenCodeGoProvider::new(
            "https://opencode.ai/zen/go/v1".to_string(),
            "test_key".to_string(),
            "deepseek-v4-flash".to_string(),
        );

        provider.set_model("new-model").unwrap();

        let forked = provider.fork();
        assert_eq!(forked.name(), "opencode-go");
        assert_eq!(forked.model(), "new-model");
    }
}
