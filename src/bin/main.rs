// src/main.rs

// dependencies
use clap::Parser;
use hello_openrouter::FireCrawlClient;
use hello_openrouter::error::AppError;
use hello_openrouter::openrouter_client::OpenRouterClient;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "anthropic/claude-sonnet-4.6")]
    model: String,

    #[arg(short, long)]
    template: String,

    #[arg(short, long)]
    source: String,

    #[cfg(feature = "vision")]
    #[arg(short, long)]
    image: Option<String>,

    #[arg(short, long, default_value = "output.md")]
    output: String,
}

async fn run(
    openrouter_api_key: &str,
    firecrawl_api_key: &str,
    args: &Args,
) -> Result<(), AppError> {
    let openrouter_client = OpenRouterClient::new(openrouter_api_key.to_string());

    let system_prompt = fs::read_to_string(&args.template)?;
    let user_prompt = if args.source.contains("https://") {
        let fire_crawl_response = FireCrawlClient::new(firecrawl_api_key.to_string())
            .scrape(&args.source)
            .await?;
        fire_crawl_response.data.markdown
    } else {
        fs::read_to_string(&args.source)?
    };

    #[cfg(feature = "vision")]
    let openrouter_client_response = if let Some(ref image_path) = args.image {
        openrouter_client
            .chat_with_image(&args.model, &user_prompt, &system_prompt, image_path)
            .await?
    } else {
        openrouter_client
            .chat(&args.model, &user_prompt, &system_prompt)
            .await?
    };

    #[cfg(not(feature = "vision"))]
    let openrouter_client_response = openrouter_client
        .chat(&args.model, &user_prompt, &system_prompt)
        .await?;

    if let Some(parent) = Path::new(&args.output).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&args.output)?;

    writeln!(file, "{}", openrouter_client_response.content())?;

    println!("Results in the file {}", args.output);
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
    dotenvy::dotenv().ok();

    let openrouter_api_key = match env::var("OPENROUTER_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let firecrawl_api_key = match env::var("FIRECRAWL_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let args = Args::parse();

    if let Err(e) = run(&openrouter_api_key, &firecrawl_api_key, &args).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
