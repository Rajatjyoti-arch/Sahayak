use agentic_harness::{
    BoxFuture, DefaultPolicy, Message, MockModel, Model, ModelError, StopReason, Tool, ToolCall,
    ToolDefinition, run_agent,
};
use sahayak_core::ReconcileTool;
use serde_json::json;
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn test_reconcile_tool_direct_execution() {
    let tool = ReconcileTool::new();
    assert_eq!(tool.name(), "reconcile");

    let result = tool.run(json!({ "scheme": "PMAY" })).await;
    assert!(result.is_ok());

    let report = result.unwrap();
    // Verify row conservation invariant verified
    assert!(report.contains("MATHEMATICALLY_CONSERVED (100% VERIFIED)"));
    assert!(
        report.contains("Total=13 [Matched=5, Unmatched=3, Duplicates=1, Invalid=1, Pending=3]")
    );

    // Verify financial bridge in integer paise
    assert!(report.contains("Total Sanctioned Amount: ₹ 56,000.50"));
    assert!(report.contains("Total Disbursed Amount:  ₹ 54,500.50"));
    assert!(report.contains("Net Scheme Variance:     ₹ 1,500.00"));
    assert!(report.contains("Zero IEEE 754 Float Error: VERIFIED"));

    // Verify root cause discrepancy identified
    assert!(report.contains("Parveen Akhtar"));
    assert!(report.contains("SAN-009"));
    assert!(report.contains("DISB-009"));
}

#[tokio::test]
async fn test_agent_loop_deterministic_mock_run() {
    let turn1 = Message::assistant(
        Some("Analyzing scheme reconciliation status...".to_string()),
        vec![ToolCall {
            id: "call_reconcile_pmay".to_string(),
            name: "reconcile".to_string(),
            arguments: json!({ "scheme": "PMAY" }),
        }],
    );
    let turn2 = Message::assistant_text(
        "Reconciliation analysis complete:\n\
        1. Row Conservation: Both files strictly conserve all 13 rows.\n\
        2. Financial Bridge: net variance of ₹ 1,500.00.\n\
        3. Root Cause: Parveen Akhtar (SAN-009) under-disbursement.",
    );

    let model = MockModel::with_responses(vec![turn1, turn2]);
    let tools: Vec<Box<dyn Tool>> = vec![Box::new(ReconcileTool::new())];
    let task = "Explain the financial variance in PMAY scheme and verify row conservation.";
    let policy = DefaultPolicy::new(".");
    let cancel = CancellationToken::new();

    let output = run_agent(&model, &tools, task, &policy, 8, cancel)
        .await
        .unwrap();

    assert_eq!(output.stop_reason, StopReason::Stop);
    assert_eq!(output.turns_executed, 2);

    // Verify full transcript structure:
    // [0] System, [1] User, [2] Assistant (Tool Call), [3] Tool Result, [4] Assistant (Final Answer)
    assert_eq!(output.history.len(), 5);

    match &output.history[1] {
        Message::User { content } => assert_eq!(content, task),
        _ => panic!("Expected user message at index 1"),
    }

    match &output.history[2] {
        Message::Assistant { tool_calls, .. } => {
            assert_eq!(tool_calls.len(), 1);
            assert_eq!(tool_calls[0].name, "reconcile");
            assert_eq!(tool_calls[0].arguments, json!({ "scheme": "PMAY" }));
        }
        _ => panic!("Expected assistant tool call message at index 2"),
    }

    match &output.history[3] {
        Message::ToolResult {
            tool_call_id,
            content,
            is_error,
        } => {
            assert_eq!(tool_call_id, "call_1_0");
            assert!(!is_error);
            assert!(content.contains("MATHEMATICALLY_CONSERVED"));
            assert!(content.contains("₹ 1,500.00"));
        }
        _ => panic!("Expected tool result message at index 3"),
    }

    match &output.history[4] {
        Message::Assistant {
            content,
            tool_calls,
            ..
        } => {
            assert!(tool_calls.is_empty());
            let text = content.as_ref().unwrap();
            assert!(text.contains("strictly conserve all 13 rows"));
            assert!(text.contains("net variance of ₹ 1,500.00"));
            assert!(text.contains("Parveen Akhtar (SAN-009)"));
        }
        _ => panic!("Expected assistant final message at index 4"),
    }
}

struct InfiniteLoopModel;
impl Model for InfiniteLoopModel {
    fn respond<'a>(
        &'a self,
        _history: &'a [Message],
        _tools: &'a [ToolDefinition],
    ) -> BoxFuture<'a, Result<Message, ModelError>> {
        Box::pin(async move {
            Ok(Message::assistant(
                Some("Calling tool repeatedly...".to_string()),
                vec![ToolCall {
                    id: "loop_call".to_string(),
                    name: "reconcile".to_string(),
                    arguments: json!({ "scheme": "PMAY" }),
                }],
            ))
        })
    }
}

#[tokio::test]
async fn test_agent_loop_max_turns_exhaustion() {
    let model = InfiniteLoopModel;
    let tools: Vec<Box<dyn Tool>> = vec![Box::new(ReconcileTool::new())];
    let policy = DefaultPolicy::new(".");
    let cancel = CancellationToken::new();

    let max_turns = 3;
    let output = run_agent(&model, &tools, "Loop test", &policy, max_turns, cancel)
        .await
        .unwrap();

    assert_eq!(output.stop_reason, StopReason::MaxTurns);
    assert_eq!(output.turns_executed, max_turns);
}
