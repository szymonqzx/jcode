//! Windsurf provider implementation
//!
//! This module provides integration with Windsurf/Codeium's local language server
//! via gRPC, allowing access to models like claude-opus-4-5-thinking and gemini-3-pro
//! with a Windsurf subscription.
//!
//! Based on: https://github.com/rsvedant/opencode-windsurf-auth

use super::{EventStream, Provider};
use crate::auth::windsurf as windsurf_auth;
use crate::message::{Message, StreamEvent, ToolDefinition};
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

const DEFAULT_MODEL: &str = "claude-4.5-opus-thinking";

/// Available Windsurf models
pub(crate) const AVAILABLE_MODELS: &[&str] = &[
    "claude-4.5-opus-thinking",
    "gpt-5.1-codex-max",
    "gemini-3.0-pro",
    "minimax-m2.1",
    "glm-4.7",
    "glm-4.7-fast",
];

/// Model enum values for Windsurf gRPC
const MODEL_ENUMS: &[(&str, &str)] = &[
    ("claude-4.5-opus-thinking", "CLAUDE_4_5_OPUS_THINKING"),
    ("gpt-5.1-codex-max", "GPT_5_1_CODEX_MAX"),
    ("gemini-3.0-pro", "GEMINI_3_0_PRO"),
    ("minimax-m2.1", "MINIMAX_M2_1"),
    ("glm-4.7", "GLM_4_7"),
    ("glm-4.7-fast", "GLM_4_7_FAST"),
];

/// Resolve model name to enum value
fn resolve_model_enum(model: &str) -> &str {
    MODEL_ENUMS
        .iter()
        .find(|(name, _)| *name == model)
        .map(|(_, enum_val)| *enum_val)
        .unwrap_or("UNKNOWN_MODEL")
}

/// Windsurf provider
pub struct WindsurfProvider {
    model: Arc<RwLock<String>>,
    credentials: windsurf_auth::WindsurfCredentials,
}

impl WindsurfProvider {
    /// Create a new Windsurf provider
    pub fn new(model: String) -> Result<Self> {
        let credentials = windsurf_auth::load_credentials()
            .context("Failed to load Windsurf credentials")?;

        Ok(Self {
            model: Arc::new(RwLock::new(model)),
            credentials,
        })
    }

    /// Get the current model
    pub fn model(&self) -> String {
        self.model.read().unwrap().clone()
    }

    /// Set the model
    pub fn set_model(&self, model: String) {
        *self.model.write().unwrap() = model;
    }

    /// Build gRPC request for Windsurf
    fn build_grpc_request(&self, messages: &[Message]) -> Result<Vec<u8>> {
        let mut request = format!(
            r#"{{"modelEnum":"{}","apiKey":"{}","messages":["#,
            resolve_model_enum(&self.model()),
            self.credentials.api_key.as_deref().unwrap_or("")
        );

        for msg in messages {
            let role = match msg.role {
                crate::message::Role::User => "user",
                crate::message::Role::Assistant => "assistant",
            };
            let content: String = msg.content.iter()
                .filter_map(|block| {
                    if let crate::message::ContentBlock::Text { text, .. } = block {
                        Some(text.clone())
                    } else {
                        None
                    }
                })
                .collect();
            request.push_str(&format!(r#"{{"role":"{}","content":"{}"}},"#, role, content.replace('"', "\\\"")));
        }

        request.push_str("]}");
        Ok(request.into_bytes())
    }

    /// Build gRPC request for Windsurf (static helper)
    fn build_grpc_request_static(model: &str, credentials: &windsurf_auth::WindsurfCredentials, messages: &[Message]) -> Result<Vec<u8>> {
        let model_enum = resolve_model_enum(model);

        // Build a simple request structure
        let mut request = format!(
            r#"{{"modelEnum":"{}","apiKey":"{}","messages":["#,
            model_enum, credentials.api_key.as_deref().unwrap_or("")
        );

        // Add messages
        for msg in messages {
            let role = match msg.role {
                crate::message::Role::User => "user",
                crate::message::Role::Assistant => "assistant",
            };
            // Extract text from content blocks
            let content: String = msg.content.iter()
                .filter_map(|block| {
                    if let crate::message::ContentBlock::Text { text, .. } = block {
                        Some(text.clone())
                    } else {
                        None
                    }
                })
                .collect();
            request.push_str(&format!(r#"{{"role":"{}","content":"{}"}},"#, role, content.replace('"', "\\\"")));
        }

        request.push_str("]}");

        Ok(request.into_bytes())
    }

    /// Parse gRPC response chunk
    fn parse_response_chunk(&self, chunk: &[u8]) -> Result<String> {
        // gRPC frame: 1 byte compression flag + 4 bytes message length + message
        if chunk.len() < 5 {
            return Ok(String::new());
        }

        let compressed = chunk[0];
        let message_length = u32::from_be_bytes([chunk[1], chunk[2], chunk[3], chunk[4]]) as usize;

        if compressed != 0 {
            // Compressed data not supported
            return Ok(String::new());
        }

        if chunk.len() < 5 + message_length {
            return Ok(String::new());
        }

        let message_data = &chunk[5..5 + message_length];

        // Simplified parsing - extract text from protobuf
        // In a full implementation, this would properly parse the protobuf structure
        String::from_utf8(message_data.to_vec())
            .context("Failed to parse response as UTF-8")
    }
}

#[async_trait]
impl Provider for WindsurfProvider {
    async fn complete(
        &self,
        messages: &[Message],
        _tools: &[ToolDefinition],
        _system: &str,
        _resume_session_id: Option<&str>,
    ) -> Result<EventStream> {
        let (tx, rx) = mpsc::channel(100);

        // Clone credentials and messages for the async task
        let credentials = self.credentials.clone();
        let model = self.model();
        let messages = messages.to_vec();

        tokio::spawn(async move {
            // Build gRPC request
            let request_body = match Self::build_grpc_request_static(&model, &credentials, &messages) {
                Ok(body) => body,
                Err(e) => {
                    let _ = tx.send(Err(e)).await;
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
                    let _ = tx.send(Err(anyhow::anyhow!("Failed to create HTTP/2 client: {}", e))).await;
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
                    let _ = tx.send(Err(anyhow::anyhow!("Failed to connect to Windsurf: {}", e))).await;
                    return;
                }
            };

            if !response.status().is_success() {
                let status = response.status();
                let body = match response.text().await {
                    Ok(b) => b,
                    Err(_) => "unknown".to_string(),
                };
                let _ = tx.send(Err(anyhow::anyhow!("Windsurf returned error {}: {}", status, body))).await;
                return;
            }

            // Stream the response
            let mut stream = response.bytes_stream();

            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        // Parse and send text chunks
                        let text = String::from_utf8_lossy(&chunk).to_string();
                        if !text.is_empty() {
                            let _ = tx.send(Ok(StreamEvent::TextDelta(text))).await;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(anyhow::anyhow!("Stream error: {}", e))).await;
                        break;
                    }
                }
            }

            let _ = tx.send(Ok(StreamEvent::MessageEnd { stop_reason: None })).await;
        });

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    fn fork(&self) -> Arc<dyn Provider> {
        Arc::new(Self {
            model: Arc::clone(&self.model),
            credentials: self.credentials.clone(),
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
}

/// Check if a model name is a known Windsurf model
pub fn is_known_model(model: &str) -> bool {
    let trimmed = model.trim();
    AVAILABLE_MODELS.contains(&trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_models() {
        assert!(!AVAILABLE_MODELS.is_empty());
        assert!(AVAILABLE_MODELS.contains(&"claude-4.5-opus-thinking"));
        assert!(AVAILABLE_MODELS.contains(&"gemini-3.0-pro"));
    }

    #[test]
    fn test_is_known_model() {
        assert!(is_known_model("claude-4.5-opus-thinking"));
        assert!(is_known_model("gemini-3.0-pro"));
        assert!(!is_known_model("unknown-model"));
        assert!(is_known_model("  claude-4.5-opus-thinking  ")); // trimmed
    }

    #[test]
    fn test_provider_creation() {
        // Try to create a Windsurf provider - will succeed if Windsurf is running
        let result = WindsurfProvider::new("claude-4.5-opus-thinking".to_string());
        match result {
            Ok(provider) => {
                println!("Windsurf provider created successfully!");
                println!("Model: {}", provider.model());
                println!("Available models: {:?}", provider.available_models());
            }
            Err(e) => {
                println!("Windsurf provider creation failed (expected if Windsurf not running): {}", e);
            }
        }
        // Test passes regardless - we're just checking it doesn't crash
    }
}
