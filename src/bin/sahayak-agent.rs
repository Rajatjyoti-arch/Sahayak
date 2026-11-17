use sahayak_core::{MockModel, Model, OllamaModel, ReconcileTool, Tool, run_agent};
use std::env;

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
        Box::new(MockModel::new())
    } else {
        println!(
            "==> Running with OLLAMA model '{}' at {}",
            ollama_model, ollama_base_url
        );
        Box::new(OllamaModel::new(ollama_base_url, ollama_model))
    };

    let tools: Vec<Box<dyn Tool>> = vec![Box::new(ReconcileTool::new())];

    let output = run_agent(model.as_ref(), &tools, &task, 8).await?;

    println!("\n========================================================");
    println!("SAHAYAK AGENT EXECUTION SUMMARY");
    println!("Stop Reason: {:?}", output.stop_reason);
    println!("Turns Executed: {}", output.turns_executed);
    println!("Total Transcript Messages: {}", output.history.len());
    println!("========================================================");

    Ok(())
}
