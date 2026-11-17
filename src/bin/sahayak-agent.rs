use agentic_harness::{
    DefaultPolicy, Message, MockModel, Model, OllamaModel, Tool, ToolCall, run_agent,
};
use sahayak_core::ReconcileTool;
use serde_json::json;
use std::env;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut use_mock = true;
    let mut ollama_model = String::new();
    let mut ollama_base_url = "http://localhost:11434".to_string();
    let mut task_parts = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--mock" => {
                use_mock = true;
                i += 1;
            }
            "--ollama" => {
                use_mock = false;
                if i + 1 < args.len() {
                    ollama_model = args[i + 1].clone();
                    i += 2;
                } else {
                    ollama_model = "llama3.2".to_string();
                    i += 1;
                }
            }
            "--base-url" => {
                if i + 1 < args.len() {
                    ollama_base_url = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--help" | "-h" => {
                println!(
                    "Sahayak Sovereign Governance Agent\n\n\
                    Usage:\n  \
                    sahayak-agent [OPTIONS] [TASK]\n\n\
                    Options:\n  \
                    --mock              Use deterministic offline mock model (default, zero network)\n  \
                    --ollama [MODEL]    Use local Ollama model (default: llama3.2)\n  \
                    --base-url <URL>    Ollama endpoint URL (default: http://localhost:11434)\n  \
                    --help, -h          Display this help message\n\n\
                    Example:\n  \
                    sahayak-agent --mock \"Explain the variance in PMAY scheme\""
                );
                return Ok(());
            }
            arg => {
                task_parts.push(arg.to_string());
                i += 1;
            }
        }
    }

    let task = if task_parts.is_empty() {
        "Explain the financial variance in PMAY scheme and verify mathematical row conservation."
            .to_string()
    } else {
        task_parts.join(" ")
    };

    let model: Box<dyn Model> = if use_mock {
        println!("==> Running in deterministic OFFLINE MOCK mode (Demo Insurance)");
        let turn1 = Message::assistant(
            Some(
                "Analyzing scheme reconciliation status and verifying row conservation. \
                Invoking the core reconciliation engine..."
                    .to_string(),
            ),
            vec![ToolCall {
                id: "call_reconcile_pmay".to_string(),
                name: "reconcile".to_string(),
                arguments: json!({ "scheme": "PMAY" }),
            }],
        );
        let turn2 = Message::assistant_text(
            "Reconciliation analysis complete:\n\
            1. Row Conservation: Both files strictly conserve all 13 rows \
            (5 Matched, 2 Unmatched, 1 Duplicate, 1 Invalid, 4 Pending). Zero silent drops.\n\
            2. Financial Bridge: Total Sanctioned is ₹ 56,000.50 (5,600,050 paise) and \
            Total Disbursed is ₹ 54,500.50 (5,450,050 paise), leaving a net variance of \
            ₹ 1,500.00 (150,000 paise). Zero floating-point error.\n\
            3. Root Cause: Beneficiary Parveen Akhtar (SAN-009) was sanctioned ₹ 5,000.00 \
            but disbursed ₹ 3,500.00 (DISB-009). The ₹ 1,500.00 under-disbursement is safely \
            isolated in the Pending partition for administrative adjudication.",
        );
        Box::new(MockModel::with_responses(vec![turn1, turn2]))
    } else {
        println!(
            "==> Running with OLLAMA model '{}' at {}",
            ollama_model, ollama_base_url
        );
        Box::new(OllamaModel::new(ollama_base_url, ollama_model))
    };

    let cwd = env::current_dir().unwrap_or_else(|_| ".".into());
    let policy = DefaultPolicy::new(cwd);
    let tools: Vec<Box<dyn Tool>> = vec![Box::new(ReconcileTool::new())];
    let cancel = CancellationToken::new();

    let output = run_agent(model.as_ref(), &tools, &task, &policy, 8, cancel).await?;

    println!("\n========================================================");
    println!("SAHAYAK AGENT EXECUTION SUMMARY");
    println!("Stop Reason: {:?}", output.stop_reason);
    println!("Turns Executed: {}", output.turns_executed);
    println!("Total Transcript Messages: {}", output.history.len());
    println!("========================================================");

    Ok(())
}
