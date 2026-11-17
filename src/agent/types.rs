use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a model-directed tool invocation request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique identifier for this tool call execution.
    pub id: String,
    /// Function/tool name to invoke (e.g., "reconcile").
    pub name: String,
    /// JSON arguments passed by the model.
    pub arguments: Value,
}

/// A conversation turn in the agent execution trace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {
    /// System prompt setting agent instructions and persona.
    System { content: String },
    /// User query or input prompt.
    User { content: String },
    /// Model response, containing text and/or tool invocation requests.
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
    },
    /// Result returned to the model after tool execution.
    #[serde(rename = "tool")]
    ToolResult {
        tool_call_id: String,
        content: String,
        #[serde(default)]
        is_error: bool,
    },
}

impl Message {
    /// Creates a system message.
    pub fn system(content: impl Into<String>) -> Self {
        Self::System {
            content: content.into(),
        }
    }

    /// Creates a user message.
    pub fn user(content: impl Into<String>) -> Self {
        Self::User {
            content: content.into(),
        }
    }

    /// Creates an assistant message with optional content and tool calls.
    pub fn assistant(content: Option<String>, tool_calls: Vec<ToolCall>) -> Self {
        Self::Assistant {
            content,
            tool_calls,
        }
    }

    /// Creates an assistant message with text only.
    pub fn assistant_text(content: impl Into<String>) -> Self {
        Self::Assistant {
            content: Some(content.into()),
            tool_calls: Vec::new(),
        }
    }

    /// Creates a tool execution result message.
    pub fn tool_result(
        tool_call_id: impl Into<String>,
        content: impl Into<String>,
        is_error: bool,
    ) -> Self {
        Self::ToolResult {
            tool_call_id: tool_call_id.into(),
            content: content.into(),
            is_error,
        }
    }
}

/// The termination condition for an agent loop invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    /// Assistant completed response without requesting further tools.
    Stop,
    /// Model requested tool executions.
    ToolUse,
    /// Loop reached configured maximum turn budget.
    MaxTurns,
    /// Loop stopped due to an unrecoverable model or tool error.
    Error,
}

/// Schema definition of a tool made available to the model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}
