use crate::agent::tool::BoxFuture;
use crate::agent::types::{Message, ToolCall, ToolDefinition};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Errors encountered during model communication or response decoding.
#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("HTTP transport error: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Model protocol error: {0}")]
    Protocol(String),
}

/// Abstract contract for language model providers.
pub trait Model: Send + Sync {
    /// Generates the next response message given conversation history and available tools.
    fn respond<'a>(
        &'a self,
        history: &'a [Message],
        tools: &'a [ToolDefinition],
    ) -> BoxFuture<'a, Result<Message, ModelError>>;
}

/// Deterministic mock model for offline execution, testing, and guaranteed hackathon demo insurance.
///
/// Implements a predictable two-turn sequence:
/// 1. Proposes a `reconcile` tool call for scheme analysis.
/// 2. Explains the audit report and row conservation results.
#[derive(Debug, Default)]
pub struct MockModel {
    invocation_count: AtomicUsize,
}

impl MockModel {
    pub fn new() -> Self {
        Self {
            invocation_count: AtomicUsize::new(0),
        }
    }
}

impl Model for MockModel {
    fn respond<'a>(
        &'a self,
        history: &'a [Message],
        _tools: &'a [ToolDefinition],
    ) -> BoxFuture<'a, Result<Message, ModelError>> {
        Box::pin(async move {
            let count = self.invocation_count.fetch_add(1, Ordering::SeqCst);

            // Check if there is already a ToolResult in history
            let has_tool_result = history
                .iter()
                .any(|m| matches!(m, Message::ToolResult { .. }));

            if !has_tool_result && count == 0 {
                // Turn 1: Propose reconcile tool call
                Ok(Message::Assistant {
                    content: Some(
                        "Analyzing scheme reconciliation status and verifying row conservation. \
                        Invoking the core reconciliation engine..."
                            .to_string(),
                    ),
                    tool_calls: vec![ToolCall {
                        id: "call_reconcile_pmay".to_string(),
                        name: "reconcile".to_string(),
                        arguments: json!({ "scheme": "PMAY" }),
                    }],
                })
            } else {
                // Turn 2: Synthesize findings and explain variance
                Ok(Message::Assistant {
                    content: Some(
                        "Reconciliation analysis complete:\n\
                        1. Row Conservation: Both files strictly conserve all 13 rows \
                        (5 Matched, 2 Unmatched, 1 Duplicate, 1 Invalid, 4 Pending). Zero silent drops.\n\
                        2. Financial Bridge: Total Sanctioned is ₹ 56,000.50 (5,600,050 paise) and \
                        Total Disbursed is ₹ 54,500.50 (5,450,050 paise), leaving a net variance of \
                        ₹ 1,500.00 (150,000 paise). Zero floating-point error.\n\
                        3. Root Cause: Beneficiary Parveen Akhtar (SAN-009) was sanctioned ₹ 5,000.00 \
                        but disbursed ₹ 3,500.00 (DISB-009). The ₹ 1,500.00 under-disbursement is safely \
                        isolated in the Pending partition for administrative adjudication."
                            .to_string(),
                    ),
                    tool_calls: Vec::new(),
                })
            }
        })
    }
}

/// Ollama local language model provider communicating over standard HTTP `/api/chat`.
pub struct OllamaModel {
    client: reqwest::Client,
    base_url: String,
    model_name: String,
}

impl OllamaModel {
    pub fn new(base_url: impl Into<String>, model_name: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
            model_name: model_name.into(),
        }
    }
}

#[derive(Serialize)]
struct OllamaChatRequest<'a> {
    model: &'a str,
    messages: Vec<Value>,
    stream: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<Value>,
}

#[derive(Deserialize)]
struct OllamaFunctionCall {
    name: String,
    arguments: Value,
}

#[derive(Deserialize)]
struct OllamaToolCall {
    function: OllamaFunctionCall,
}

#[derive(Deserialize)]
struct OllamaMessageResponse {
    content: Option<String>,
    #[serde(default)]
    tool_calls: Vec<OllamaToolCall>,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: OllamaMessageResponse,
}

impl Model for OllamaModel {
    fn respond<'a>(
        &'a self,
        history: &'a [Message],
        tools: &'a [ToolDefinition],
    ) -> BoxFuture<'a, Result<Message, ModelError>> {
        Box::pin(async move {
            let endpoint = format!("{}/api/chat", self.base_url.trim_end_matches('/'));

            // Convert internal messages to Ollama format
            let mut messages_payload = Vec::new();
            for msg in history {
                match msg {
                    Message::System { content } => {
                        messages_payload.push(json!({
                            "role": "system",
                            "content": content,
                        }));
                    }
                    Message::User { content } => {
                        messages_payload.push(json!({
                            "role": "user",
                            "content": content,
                        }));
                    }
                    Message::Assistant {
                        content,
                        tool_calls,
                    } => {
                        if tool_calls.is_empty() {
                            messages_payload.push(json!({
                                "role": "assistant",
                                "content": content.clone().unwrap_or_default(),
                            }));
                        } else {
                            let ollama_tools: Vec<Value> = tool_calls
                                .iter()
                                .map(|tc| {
                                    json!({
                                        "function": {
                                            "name": tc.name,
                                            "arguments": tc.arguments,
                                        }
                                    })
                                })
                                .collect();
                            messages_payload.push(json!({
                                "role": "assistant",
                                "content": content.clone().unwrap_or_default(),
                                "tool_calls": ollama_tools,
                            }));
                        }
                    }
                    Message::ToolResult { content, .. } => {
                        messages_payload.push(json!({
                            "role": "tool",
                            "content": content,
                        }));
                    }
                }
            }

            // Format tools into Ollama schema
            let tools_payload: Vec<Value> = tools
                .iter()
                .map(|t| {
                    json!({
                        "type": "function",
                        "function": {
                            "name": t.name,
                            "description": t.description,
                            "parameters": t.parameters,
                        }
                    })
                })
                .collect();

            let req = OllamaChatRequest {
                model: &self.model_name,
                messages: messages_payload,
                stream: false,
                tools: tools_payload,
            };

            let res = self
                .client
                .post(&endpoint)
                .json(&req)
                .send()
                .await?
                .error_for_status()?;

            let chat_res: OllamaChatResponse = res.json().await?;

            let tool_calls: Vec<ToolCall> = chat_res
                .message
                .tool_calls
                .into_iter()
                .enumerate()
                .map(|(idx, tc)| ToolCall {
                    id: format!("call_{idx}"),
                    name: tc.function.name,
                    arguments: tc.function.arguments,
                })
                .collect();

            Ok(Message::Assistant {
                content: chat_res.message.content,
                tool_calls,
            })
        })
    }
}
