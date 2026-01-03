// src/main.rs

// dependencies
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use thiserror::Error;

// constants
const OPENROUTER_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Error)]
enum AppError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Environment variable error: {0}")]
    Env(#[from] env::VarError),

    #[error("Failed to load .env file: {0}")]
    DotEnv(#[from] dotenvy::Error),

    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
}

async fn run() -> Result<(), AppError> {
    dotenvy::dotenv()?;

    let openrouter_api_key = env::var("OPENROUTER_API_KEY")?;

    let http_client = Client::new();

    let model = "anthropic/claude-sonnet-4.5";
    let system_prompt = fs::read_to_string("system.md")?;
    let user_prompt = fs::read_to_string("prompt.md")?;

    let body = json! {{
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": system_prompt
            },
            {
                "role": "user",
                "content": user_prompt
            }
        ]
    }};

    let response = http_client
        .post(OPENROUTER_URL)
        .header("Authorization", format!("Bearer {}", openrouter_api_key))
        .json(&body)
        .send()
        .await?;

    let chat_response = response.json::<ChatResponse>().await?;

    let answer = &chat_response.choices[0].message.content;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("output.md")?;

    writeln!(file, "{answer}")?;

    println!("Results in the file output.md");
    println!(
        "Tokens used: {} prompt, {} completion, {} total",
        chat_response.usage.prompt_tokens,
        chat_response.usage.completion_tokens,
        chat_response.usage.total_tokens
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
