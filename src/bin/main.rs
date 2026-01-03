// src/main.rs

// dependencies
use clap::Parser;
use hello_openrouter::client::OpenRouterClient;
use hello_openrouter::error::AppError;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "anthropic/claude-sonnet-4")]  
    model: String,

    #[arg(short, long)]
    system_prompt: String,

    #[arg(short, long)]
    user_prompt: String,

    #[arg(short, long, default_value = "output.md")]
    output_name: String,
}

async fn run(model: &str, user_prompt: &str, system_prompt: &str, output_name: &str) -> Result<(), AppError> {
    dotenvy::dotenv()?;

    let openrouter_api_key = env::var("OPENROUTER_API_KEY")?;

    let openrouter_client = OpenRouterClient::new(openrouter_api_key);

    let system_prompt = fs::read_to_string(system_prompt)?;
    let user_prompt = fs::read_to_string(user_prompt)?;

    let openrouter_client_response = openrouter_client
        .chat(model, &system_prompt, &user_prompt)
        .await?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_name)?;

    writeln!(file, "{}", openrouter_client_response.content())?;

    println!("Results in the file {}", output_name);
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
    let args = Args::parse();
    
    if let Err(e) = run(&args.model, &args.system_prompt, &args.user_prompt, &args.output_name).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
