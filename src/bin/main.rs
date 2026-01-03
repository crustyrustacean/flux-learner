// src/main.rs

// dependencies
use hello_openrouter::client::OpenRouterClient;
use hello_openrouter::error::AppError;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;

async fn run() -> Result<(), AppError> {
    dotenvy::dotenv()?;

    let openrouter_api_key = env::var("OPENROUTER_API_KEY")?;

    let openrouter_client = OpenRouterClient::new(openrouter_api_key);

    let model = "anthropic/claude-sonnet-4.5";
    let system_prompt = fs::read_to_string("system.md")?;
    let user_prompt = fs::read_to_string("prompt.md")?;

    let openrouter_client_response = openrouter_client
        .chat(model, &system_prompt, &user_prompt)
        .await?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("output.md")?;

    writeln!(file, "{}", openrouter_client_response.content())?;

    println!("Results in the file output.md");
    println!(
        "Tokens used: {} prompt, {} completion, {} total",
        openrouter_client_response.prompt_tokens(),
        openrouter_client_response.completion_tokens(),
        openrouter_client_response.total_tokens(),
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
