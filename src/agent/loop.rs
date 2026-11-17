use crate::agent::model::{Model, ModelError};
use crate::agent::tool::Tool;
use crate::agent::types::{Message, StopReason, ToolDefinition};
use std::time::Instant;

/// Execution errors that occur during the agent loop.
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Model failure: {0}")]
    Model(#[from] ModelError),
}

/// The result returned after agent execution finishes.
#[derive(Debug, Clone)]
pub struct AgentOutput {
    /// Full transcript of messages exchanged during the run.
    pub history: Vec<Message>,
    /// The condition under which the run terminated.
    pub stop_reason: StopReason,
    /// Number of turns executed.
    pub turns_executed: usize,
}

/// Executes the agent loop until the assistant completes or turns are exhausted.
///
/// Each turn queries the model, executes any requested tools asynchronously in-memory,
/// appends results to history, and logs step-by-step audit trace lines to stdout.
pub async fn run(
    model: &dyn Model,
    tools: &[Box<dyn Tool>],
    task: &str,
    max_turns: usize,
) -> Result<AgentOutput, AgentError> {
    let mut history = vec![
        Message::system(
            "You are Sahayak (सहायक), an offline sovereign governance and reconciliation assistant. \
            Your mission is to perform rigorous scheme verification, protect row conservation, \
            and identify financial variances in integer paise.",
        ),
        Message::user(task),
    ];

    let tool_definitions: Vec<ToolDefinition> = tools.iter().map(|t| t.definition()).collect();

    println!(
        "[Agent Trace] Initializing Sahayak agent loop for task: \"{}\"",
        task
    );
    println!(
        "[Agent Trace] Registered tools: {:?}",
        tool_definitions.iter().map(|t| &t.name).collect::<Vec<_>>()
    );

    for turn in 1..=max_turns {
        println!("\n[Agent Trace] === Turn {} / {} ===", turn, max_turns);

        let assistant_msg = model.respond(&history, &tool_definitions).await?;

        match &assistant_msg {
            Message::Assistant {
                content,
                tool_calls,
            } => {
                if let Some(text) = content {
                    println!(
                        "[Agent Trace] Turn {}: Assistant thought/message:\n{}",
                        turn, text
                    );
                }

                history.push(assistant_msg.clone());

                if tool_calls.is_empty() {
                    println!(
                        "[Agent Trace] Turn {}: No tools requested. Task complete.",
                        turn
                    );
                    return Ok(AgentOutput {
                        history,
                        stop_reason: StopReason::Stop,
                        turns_executed: turn,
                    });
                }

                for tool_call in tool_calls {
                    println!(
                        "[Agent Trace] Turn {}: Model proposed tool '{}' (id: {}) with args: {}",
                        turn, tool_call.name, tool_call.id, tool_call.arguments
                    );

                    let matching_tool = tools.iter().find(|t| t.name() == tool_call.name);

                    let (result_content, is_error) = match matching_tool {
                        Some(tool) => {
                            let start = Instant::now();
                            match tool.run(tool_call.arguments.clone()).await {
                                Ok(out) => {
                                    let duration = start.elapsed();
                                    println!(
                                        "[Agent Trace] Turn {}: Tool '{}' executed successfully in {:?} ({} bytes output)",
                                        turn,
                                        tool_call.name,
                                        duration,
                                        out.len()
                                    );
                                    (out, false)
                                }
                                Err(err) => {
                                    println!(
                                        "[Agent Trace] Turn {}: Tool '{}' failed: {}",
                                        turn, tool_call.name, err
                                    );
                                    (format!("Tool execution error: {}", err), true)
                                }
                            }
                        }
                        None => {
                            let msg = format!("Error: tool '{}' not found", tool_call.name);
                            println!("[Agent Trace] Turn {}: {}", turn, msg);
                            (msg, true)
                        }
                    };

                    history.push(Message::tool_result(
                        &tool_call.id,
                        result_content,
                        is_error,
                    ));
                }
            }
            _ => {
                // Unexpected message type returned as assistant response
                return Ok(AgentOutput {
                    history,
                    stop_reason: StopReason::Error,
                    turns_executed: turn,
                });
            }
        }
    }

    println!(
        "[Agent Trace] Reached maximum allowed turns ({}) without final completion.",
        max_turns
    );
    Ok(AgentOutput {
        history,
        stop_reason: StopReason::MaxTurns,
        turns_executed: max_turns,
    })
}
