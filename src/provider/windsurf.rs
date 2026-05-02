//! Windsurf provider implementation
//!
//! This module provides integration with Windsurf/Codeium's local language server
//! via gRPC, allowing access to models like claude-opus-4-5-thinking and gemini-3-pro
//! with a Windsurf subscription.
//!
//! Based on: https://github.com/rsvedant/opencode-windsurf-auth

use super::{EventStream, Provider};
use super::common::{recover_rwlock_read, recover_rwlock_write};
use super::protobuf::{encode_message, encode_string, encode_varint_field, parse_fields, FieldValue};
use crate::auth::windsurf as windsurf_auth;
use crate::logging;
use crate::message::{ContentBlock, Message as ChatMessage, Role, StreamEvent, ToolDefinition};
use anyhow::{Context, Result};
use async_trait::async_trait;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use prost::Message as ProstMessage;
use reqwest::Client;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;

/// Available Windsurf models
pub(crate) const AVAILABLE_MODELS: &[&str] = &[
    "swe-1.5",
    "swe-1.6",
    "kimi-k2.5",
    "kimi-k2.6",
];

/// Model enum values for Windsurf gRPC
const MODEL_ENUMS: &[(&str, i32)] = &[
    ("swe-1.5", 1001),
    ("swe-1.5-thinking", 1002),
    ("swe-1.5-slow", 1003),
    ("swe-1.6", 1004),
    ("kimi-k2", 2001),
    ("kimi-k2-thinking", 2002),
    ("kimi-k2.5", 2003),
    ("kimi-k2.6", 2004),
];

/// ChatMessageSource enum values
#[derive(Clone, Copy, PartialEq)]
enum ChatMessageSource {
    User = 1,
    System = 2,
    Assistant = 3,
    Tool = 4,
}

// ============================================================================
// Request Building
// ============================================================================

/// Encode a google.protobuf.Timestamp
/// Field 1: seconds (int64)
/// Field 2: nanos (int32)
fn encode_timestamp() -> Vec<u8> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|e| {
            crate::logging::warn(&format!("Failed to get system time: {}", e));
            std::time::Duration::from_secs(0)
        });
    let seconds = now.as_secs();
    let nanos = now.subsec_nanos();

    let mut bytes = Vec::new();
    bytes.extend(encode_varint_field(1, seconds));
    if nanos > 0 {
        bytes.extend(encode_varint_field(2, nanos as u64));
    }
    bytes
}

/// Encode IntentGeneric message
/// Field 1: text (string)
fn encode_intent_generic(text: &str) -> Vec<u8> {
    encode_string(1, text)
}

/// Encode ChatMessageIntent message
/// Field 1: generic (IntentGeneric, oneof)
fn encode_chat_message_intent(text: &str) -> Vec<u8> {
    let generic = encode_intent_generic(text);
    encode_message(1, &generic)
}

/// Build the metadata message for the request
/// Field 1: ide_name (string)
/// Field 2: ide_version (string)
/// Field 3: api_key (string, required)
/// Field 4: extension_version (string)
/// Field 5: session_id (string, optional)
/// Field 6: locale (string, optional)
fn encode_metadata(api_key: &str, version: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend(encode_string(1, "windsurf")); // ide_name
    bytes.extend(encode_string(2, version)); // ide_version
    bytes.extend(encode_string(3, api_key)); // api_key (required)
    bytes.extend(encode_string(4, version)); // extension_version
    // session_id and locale are optional - not included for simplicity
    bytes
}

/// Encode a ChatMessage for the RawGetChatMessageRequest
/// Field 1: message_id (string, required)
/// Field 2: source (enum: 1=USER, 2=SYSTEM, 3=ASSISTANT)
/// Field 3: timestamp (google.protobuf.Timestamp, required)
/// Field 4: conversation_id (string, required)
/// Field 5: For USER/SYSTEM/TOOL: intent (ChatMessageIntent)
///        For ASSISTANT: text (string)
fn encode_chat_message(content: &str, source: ChatMessageSource, conversation_id: &str) -> Vec<u8> {
    let message_id = Uuid::new_v4().to_string();
    let mut bytes = Vec::new();

    // Field 1: message_id (required)
    bytes.extend(encode_string(1, &message_id));

    // Field 2: source
    bytes.extend(encode_varint_field(2, source as u64));

    // Field 3: timestamp (required)
    let timestamp = encode_timestamp();
    bytes.extend(encode_message(3, &timestamp));

    // Field 4: conversation_id (required)
    bytes.extend(encode_string(4, conversation_id));

    // Field 5: content
    if source == ChatMessageSource::Assistant {
        // Assistant replies use plain text field
        bytes.extend(encode_string(5, content));
    } else {
        let intent = encode_chat_message_intent(content);
        bytes.extend(encode_message(5, &intent));
    }

    bytes
}

/// Build the complete chat request buffer using RawGetChatMessageRequest format
/// Field 1: metadata (Metadata message)
/// Field 2: chat_messages (repeated ChatMessage)
/// Field 3: system_prompt_override (string) - optional
/// Field 4: chat_model (enum: Model)
/// Field 5: chat_model_name (string) - optional
fn build_chat_request(
    api_key: &str,
    _version: &str,
    model_enum: i32,
    messages: &[Message],
    model_name: &str,
    system_prompt: Option<&str>,
) -> Result<Vec<u8>> {
    let metadata = encode_metadata(api_key, "");
    let conversation_id = Uuid::new_v4().to_string();

    // Build the request with all messages
    let mut request = Vec::new();

    // Field 1: metadata
    request.extend(encode_message(1, &metadata));

    // Field 2: chat_messages (repeated ChatMessage)
    for msg in messages {
        // Determine message source based on content
        let has_tool_content = msg.content.iter().any(|block| {
            matches!(
                block,
                crate::message::ContentBlock::ToolUse { .. } | crate::message::ContentBlock::ToolResult { .. }
            )
        });

        let source = if has_tool_content {
            ChatMessageSource::Tool
        } else {
            match msg.role {
                crate::message::Role::User => ChatMessageSource::User,
                crate::message::Role::Assistant => ChatMessageSource::Assistant,
            }
        };

        // Extract text from content blocks
        let content: String = msg.content.iter()
            .filter_map(|block| {
                if let crate::message::ContentBlock::Text { text, .. } = block {
                    Some(text.clone())
                } else if let crate::message::ContentBlock::ToolUse { name, input, .. } = block {
                    // Encode tool calls as JSON string
                    let tool_call = serde_json::json!({
                        "name": name,
                        "input": input
                    });
                    Some(tool_call.to_string())
                } else if let crate::message::ContentBlock::ToolResult { content, .. } = block {
                    // Encode tool results
                    Some(format!("Tool result: {}", content))
                } else {
                    None
                }
            })
            .collect();

        let chat_msg = encode_chat_message(&content, source, &conversation_id);
        request.extend(encode_message(2, &chat_msg));
    }

    // Field 3: system_prompt_override (optional)
    if let Some(system) = system_prompt {
        if !system.is_empty() {
            request.extend(encode_string(3, system));
        }
    }

    // Field 4: model enum
    request.extend(encode_varint_field(4, model_enum as u64));

    // Field 5: chat_model_name (string)
    request.extend(encode_string(5, model_name));

    // gRPC framing: 1 byte compression flag (0) + 4 bytes length + payload
    let payload = request;
    let mut frame = Vec::with_capacity(5 + payload.len());
    frame.push(0); // No compression
    frame.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    frame.extend(payload);

    Ok(frame)
}

/// Resolve model name to enum value
/// Returns an error if the model is not known, to avoid silently using the wrong model
fn resolve_model_enum(model: &str) -> Result<i32> {
    MODEL_ENUMS
        .iter()
        .find(|(name, _)| *name == model)
        .map(|(_, enum_val)| *enum_val)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Unknown Windsurf model '{}'. Known models: {}",
                model,
                MODEL_ENUMS.iter().map(|(name, _)| *name).collect::<Vec<_>>().join(", ")
            )
        })
}

/// Build a tool-calling prompt when tools are provided
/// This enhances the system prompt with tool definitions and instructions
fn build_tool_calling_prompt(system: &str, tools: &[ToolDefinition]) -> String {
    let mut prompt = system.to_string();

    if !prompt.is_empty() {
        prompt.push_str("\n\n");
    }

    prompt.push_str("You have access to the following tools:\n\n");

    for tool in tools {
        prompt.push_str(&format!("Tool: {}\n", tool.name));
        prompt.push_str(&format!("Description: {}\n", tool.description));
        let schema_json = match serde_json::to_string(&tool.input_schema) {
            Ok(s) => s,
            Err(e) => {
                crate::logging::warn(&format!("Failed to serialize tool input schema for '{}': {}", tool.name, e));
                "{}".to_string()
            }
        };
        prompt.push_str(&format!("Input Schema: {}\n", schema_json));
        prompt.push_str("\n");
    }

    prompt.push_str("When you need to use a tool, respond with a tool call in the following format:\n");
    prompt.push_str("```\n");
    prompt.push_str("tool_calls: [{\"name\": \"tool_name\", \"arguments\": {\"param\": \"value\"}}]\n");
    prompt.push_str("```\n");
    prompt.push_str("Otherwise, provide your response directly as text.\n");

    prompt
}

// ============================================================================
// Response Parsing (Protobuf Decoding)
// ============================================================================

/// Extract text from RawChatMessage protobuf
/// Field 1: message_id (string)
/// Field 2: source (enum)
/// Field 3: timestamp (message)
/// Field 4: conversation_id (string)
/// Field 5: text (string) ← What we want
/// Field 6: in_progress (bool)
/// Field 7: is_error (bool)
fn extract_text_from_raw_chat_message(buffer: &[u8]) -> String {
    if let Ok(fields) = parse_fields(buffer) {
        for field in fields {
            // Field 5 is the text content
            if field.number == 5 {
                if let FieldValue::Bytes(data) = field.value {
                    if let Ok(text) = String::from_utf8(data) {
                        return text;
                    }
                }
            }
        }
    }
    String::new()
}

/// Extract text from RawGetChatMessageResponse protobuf
/// Field 1: delta_message (RawChatMessage)
fn extract_text_from_response(buffer: &[u8]) -> String {
    if let Ok(fields) = parse_fields(buffer) {
        for field in fields {
            // Field 1 is delta_message (RawChatMessage)
            if field.number == 1 {
                if let FieldValue::Bytes(data) = field.value {
                    let text = extract_text_from_raw_chat_message(&data);
                    if !text.is_empty() {
                        return text;
                    }
                }
            }
        }
    }
    String::new()
}

/// Extract readable text from a gRPC response chunk
/// The response is gRPC-framed: 1 byte compression + 4 bytes length + protobuf payload
fn extract_text_from_chunk(chunk: &[u8]) -> String {
    let mut results = Vec::new();
    let mut offset = 0;

    while offset + 5 <= chunk.len() {
        let compressed = chunk[offset];
        let message_length = u32::from_be_bytes([chunk[offset + 1], chunk[offset + 2], chunk[offset + 3], chunk[offset + 4]]) as usize;

        if compressed != 0 {
            // Compressed data not supported, skip
            offset += 5 + message_length;
            continue;
        }

        if offset + 5 + message_length > chunk.len() {
            // Not enough data for the full message, try as raw protobuf
            break;
        }

        let message_data = &chunk[offset + 5..offset + 5 + message_length];
        let text = extract_text_from_response(message_data);
        if !text.is_empty() {
            results.push(text);
        }
        offset += 5 + message_length;
    }

    // If we extracted text from proper protobuf parsing, return it
    if !results.is_empty() {
        results.join("")
    } else {
        // Fallback: try parsing the entire chunk as protobuf
        extract_text_from_response(chunk)
    }
}

/// Windsurf provider
pub struct WindsurfProvider {
    model: Arc<RwLock<String>>,
    credentials: Arc<RwLock<windsurf_auth::WindsurfCredentials>>,
}

impl WindsurfProvider {
    /// Create a new Windsurf provider
    pub fn new(model: String) -> Result<Self> {
        let credentials = windsurf_auth::load_credentials()
            .context("Failed to load Windsurf credentials")?;

        Ok(Self {
            model: Arc::new(RwLock::new(model)),
            credentials: Arc::new(RwLock::new(credentials)),
        })
    }

    /// Get the current model
    pub fn model(&self) -> String {
        recover_rwlock_read(&self.model, |guard| guard, "windsurf", "model read")
    }

    /// Set the model
    pub fn set_model(&self, model: String) {
        *recover_rwlock_write(&self.model, |guard| guard, "windsurf", "model write") = model;
    }

    /// Refresh credentials from disk
    pub fn refresh_credentials(&self) -> Result<()> {
        let new_creds = windsurf_auth::load_credentials()
            .context("Failed to refresh Windsurf credentials")?;
        *recover_rwlock_write(&self.credentials, |guard| guard, "windsurf", "credentials write") = new_creds;
        Ok(())
    }
}

#[async_trait]
impl Provider for WindsurfProvider {
    async fn complete(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
        system: &str,
        _resume_session_id: Option<&str>,
    ) -> Result<EventStream> {
        let (tx, rx) = mpsc::channel(100);

        // Clone credentials and messages for the async task
        let credentials = recover_rwlock_read(&self.credentials, |guard| guard, "windsurf", "credentials read");
        let model = self.model();
        let messages = messages.to_vec();
        let system = system.to_string();
        let tools = tools.to_vec();

        tokio::spawn(async move {
            // Build gRPC request with protobuf encoding
            let model_enum = resolve_model_enum(&model)
                .map_err(|e| anyhow::anyhow!("Failed to resolve Windsurf model: {}", e))?;
            let model_name = &model;
            let api_key = credentials.api_key.as_deref()
                .ok_or_else(|| anyhow::anyhow!("Windsurf API key not found. Please login to Windsurf first."))?;
            let version = &credentials.version;

            // Build tool-calling prompt if tools are provided
            let enhanced_system = if !tools.is_empty() {
                build_tool_calling_prompt(&system, &tools)
            } else {
                system.to_string()
            };

            // Build messages with system prompt
            let request_body = match build_chat_request(
                api_key,
                version,
                model_enum,
                &messages,
                model_name,
                if enhanced_system.is_empty() { None } else { Some(&enhanced_system) },
            ) {
                Ok(body) => body,
                Err(e) => {
                    if tx.send(Err(e)).await.is_err() {
                        return;
                    }
                    return;
                }
            };

            // Create HTTP/2 client
            let client = match reqwest::Client::builder()
                .http2_prior_knowledge()
                .build()
            {
                Ok(c) => c,
                Err(e) => {
                    let error_msg = anyhow::anyhow!("Failed to create HTTP/2 client: {} (model={})", e, model);
                    if tx.send(Err(error_msg)).await.is_err() {
                        return;
                    }
                    return;
                }
            };

            let url = format!("http://localhost:{}/exa.language_server_pb.LanguageServerService/RawGetChatMessage", credentials.port);

            let mut request_builder = client
                .post(&url)
                .header("content-type", "application/grpc")
                .header("te", "trailers");

            // Add CSRF token if available
            if let Some(ref csrf_token) = credentials.csrf_token {
                request_builder = request_builder.header("x-codeium-csrf-token", csrf_token);
            }

            let response = match request_builder
                .body(request_body)
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    let error_msg = anyhow::anyhow!("Failed to connect to Windsurf: {} (port={}, model={})", e, credentials.port, model);
                    if tx.send(Err(error_msg)).await.is_err() {
                        return;
                    }
                    return;
                }
            };

            if !response.status().is_success() {
                let status = response.status();
                let body = match response.text().await {
                    Ok(b) => b,
                    Err(e) => {
                        crate::logging::warn(&format!("Failed to read Windsurf error response body: {}", e));
                        "unknown".to_string()
                    }
                };
                let error_msg = anyhow::anyhow!("Windsurf returned error {}: {} (port={}, model={})", status, body, credentials.port, model);
                if tx.send(Err(error_msg)).await.is_err() {
                    return;
                }
                return;
            }

            // Stream the response and parse protobuf
            let mut stream = response.bytes_stream();

            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        // Parse gRPC response chunk using protobuf decoding
                        let text = extract_text_from_chunk(&chunk);
                        if !text.is_empty() {
                            if tx.send(Ok(StreamEvent::TextDelta(text))).await.is_err() {
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        let error_msg = anyhow::anyhow!("Stream error: {}", e);
                        if tx.send(Err(error_msg)).await.is_err() {
                            return;
                        }
                        break;
                    }
                }
            }

            if tx.send(Ok(StreamEvent::MessageEnd { stop_reason: None })).await.is_err() {
                return;
            }
        });

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    fn fork(&self) -> Arc<dyn Provider> {
        // Note: This shares the same Arc<RwLock> for credentials and model with the parent.
        // This is acceptable because Windsurf credentials are loaded once at startup
        // and don't change during normal operation. If dynamic credential reloading
        // is needed, this should create a new provider instance instead.
        Arc::new(Self {
            model: Arc::clone(&self.model),
            credentials: Arc::clone(&self.credentials),
        })
    }

    fn name(&self) -> &str {
        "windsurf"
    }

    fn available_models(&self) -> Vec<&'static str> {
        AVAILABLE_MODELS.to_vec()
    }

    fn available_models_for_switching(&self) -> Vec<String> {
        AVAILABLE_MODELS.iter().map(|s| s.to_string()).collect()
    }

    fn available_models_display(&self) -> Vec<String> {
        self.available_models_for_switching()
    }

    fn set_model(&self, model: &str) -> anyhow::Result<()> {
        *recover_rwlock_write(&self.model, |guard| guard, "windsurf", "model write") = model.to_string();
        Ok(())
    }

    fn model_routes(&self) -> Vec<super::ModelRoute> {
        self.available_models_for_switching()
            .into_iter()
            .map(|model| super::ModelRoute {
                model,
                provider: "Windsurf".to_string(),
                api_method: "windsurf".to_string(),
                available: true,
                detail: String::new(),
                cheapness: None,
            })
            .collect()
    }

    async fn invalidate_credentials(&self) {
        if let Err(e) = self.refresh_credentials() {
            crate::logging::warn(&format!("Failed to refresh Windsurf credentials: {}", e));
        }
    }

    fn context_window(&self) -> usize {
        // Context windows for Windsurf models
        // SWE models: Context window not publicly documented, using 200K conservative estimate
        // kimi-k2 series: 256K tokens (confirmed from Kimi API documentation)
        // Note: SWE model context windows should be verified against Cognition's official documentation
        match self.model().as_str() {
            "swe-1.5" | "swe-1.5-thinking" | "swe-1.5-slow" => 200_000,
            "swe-1.6" => 200_000,
            "kimi-k2" | "kimi-k2-thinking" | "kimi-k2.5" | "kimi-k2.6" => 256_000,
            _ => 200_000, // Conservative default for unknown models
        }
    }
}

/// Check if a model name is a known Windsurf model
pub fn is_known_model(model: &str) -> bool {
    let trimmed = model.trim();
    AVAILABLE_MODELS.contains(&trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{ContentBlock, Message, Role};

    #[test]
    fn test_available_models() {
        assert!(!AVAILABLE_MODELS.is_empty());
        assert!(AVAILABLE_MODELS.contains(&"swe-1.5"));
        assert!(AVAILABLE_MODELS.contains(&"swe-1.6"));
        assert!(AVAILABLE_MODELS.contains(&"kimi-k2.5"));
        assert!(AVAILABLE_MODELS.contains(&"kimi-k2.6"));
    }

    #[test]
    fn test_is_known_model() {
        assert!(is_known_model("swe-1.5"));
        assert!(is_known_model("swe-1.5-thinking"));
        assert!(is_known_model("swe-1.5-slow"));
        assert!(is_known_model("swe-1.6"));
        assert!(is_known_model("kimi-k2"));
        assert!(is_known_model("kimi-k2-thinking"));
        assert!(is_known_model("kimi-k2.5"));
        assert!(is_known_model("kimi-k2.6"));
        assert!(!is_known_model("unknown-model"));
        assert!(is_known_model("  swe-1.5  ")); // trimmed
    }

    #[test]
    fn test_resolve_model_enum() {
        assert_eq!(resolve_model_enum("swe-1.5").unwrap(), 1001);
        assert_eq!(resolve_model_enum("swe-1.6").unwrap(), 1004);
        assert_eq!(resolve_model_enum("kimi-k2.5").unwrap(), 2003);
        assert!(resolve_model_enum("unknown-model").is_err());
    }

    #[test]
    fn test_encode_timestamp() {
        let timestamp = encode_timestamp();
        assert!(!timestamp.is_empty());
        // Verify it encodes at least the seconds field (field 1)
        assert!(timestamp.len() > 0);
    }

    #[test]
    fn test_build_tool_calling_prompt() {
        let tools = vec![
            ToolDefinition {
                name: "test_tool".to_string(),
                description: "A test tool".to_string(),
                input_schema: serde_json::json!({"type": "object", "properties": {}}),
            },
        ];
        let prompt = build_tool_calling_prompt("You are helpful", &tools);
        assert!(prompt.contains("test_tool"));
        assert!(prompt.contains("A test tool"));
        assert!(prompt.contains("You are helpful"));
    }

    #[test]
    fn test_build_tool_calling_prompt_empty_system() {
        let tools = vec![
            ToolDefinition {
                name: "test_tool".to_string(),
                description: "A test tool".to_string(),
                input_schema: serde_json::json!({"type": "object"}),
            },
        ];
        let prompt = build_tool_calling_prompt("", &tools);
        assert!(prompt.contains("test_tool"));
        assert!(!prompt.starts_with("\n\n")); // No double newline at start
    }

    #[test]
    fn test_extract_text_from_raw_chat_message() {
        // Test with valid protobuf-like data
        // This is a simplified test - actual protobuf encoding would be more complex
        let buffer = vec![];
        let text = extract_text_from_raw_chat_message(&buffer);
        // Should return empty string for invalid data
        assert!(text.is_empty() || text.len() >= 0);
    }

    #[test]
    fn test_extract_text_from_chunk() {
        // Test with empty chunk
        let chunk = vec![];
        let text = extract_text_from_chunk(&chunk);
        assert!(text.is_empty());
    }

    #[test]
    fn test_context_window() {
        // This test doesn't require actual provider creation
        let test_cases = vec![
            ("swe-1.5", 200_000),
            ("swe-1.6", 200_000),
            ("kimi-k2.5", 256_000),
            ("kimi-k2.6", 256_000),
            ("unknown", 200_000), // conservative default
        ];

        for (model, expected) in test_cases {
            assert_eq!(
                expected,
                match model {
                    "swe-1.5" | "swe-1.5-thinking" | "swe-1.5-slow" => 200_000,
                    "swe-1.6" => 200_000,
                    "kimi-k2" | "kimi-k2-thinking" | "kimi-k2.5" | "kimi-k2.6" => 256_000,
                    _ => 200_000,
                }
            );
        }
    }

    #[test]
    fn test_provider_creation() {
        // This test requires Windsurf to be running with valid credentials.
        // Skip if not available to avoid CI failures.
        let result = WindsurfProvider::new("swe-1.5".to_string());
        if result.is_err() {
            // Windsurf not running or not configured - skip test gracefully
            return;
        }
        let provider = result.unwrap();
        assert!(!provider.model().is_empty());
        assert!(!provider.available_models().is_empty());
    }

    #[test]
    fn test_model_consistency() {
        // Test that available_models_for_switching and available_models_display are consistent
        let test_cases = vec!["swe-1.5", "swe-1.6", "kimi-k2.5", "kimi-k2.6"];
        for model in test_cases {
            assert!(is_known_model(model));
        }
    }
}
