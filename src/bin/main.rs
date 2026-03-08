// src/main.rs

// dependencies
use clap::Parser;
use flux_learner::FireCrawlClient;
use flux_learner::error::AppError;
use flux_learner::openrouter_client::OpenRouterClient;
use flux_learner::source;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Duration;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "anthropic/claude-sonnet-4.6")]
    model: String,

    #[arg(short, long)]
    template: String,

    /// URL (fetched via Firecrawl) or local file path
    #[arg(short, long, group = "input")]
    source: Option<String>,

    /// Directory to walk recursively (respects .gitignore)
    #[arg(short, long, group = "input")]
    dir: Option<String>,

    /// Manifest file listing paths in study order
    #[arg(long, group = "input")]
    manifest: Option<String>,

    /// Glob pattern for --dir (default: Rust source files)
    #[arg(long, default_value = "**/*.rs")]
    glob: String,

    /// Patterns to exclude from --dir (repeatable)
    #[arg(long)]
    exclude: Vec<String>,

    #[cfg(feature = "vision")]
    #[arg(short, long)]
    image: Option<String>,

    #[arg(short, long, default_value = "output.md")]
    output: String,
}

async fn run(args: &Args) -> Result<(), AppError> {
    let openrouter_api_key = env::var("OPENROUTER_API_KEY")?;
    let openrouter_client = OpenRouterClient::new(openrouter_api_key);

    let system_prompt = fs::read_to_string(&args.template)?;

    let user_prompt = if let Some(ref src) = args.source {
        if src.starts_with("https://") {
            let firecrawl_api_key = env::var("FIRECRAWL_API_KEY")?;
            let fire_crawl_response = FireCrawlClient::new(firecrawl_api_key)
                .scrape(src)
                .await?;
            fire_crawl_response.data.markdown
        } else {
            fs::read_to_string(src)?
        }
    } else if let Some(ref dir) = args.dir {
        source::walk_directory(dir, &args.glob, &args.exclude)?
    } else if let Some(ref manifest) = args.manifest {
        source::read_manifest(manifest)?
    } else {
        return Err(AppError::NoInput);
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

    let args = Args::parse();

    let spinner = tokio::spawn(async {
        loop {
            print!(".");
            std::io::stdout().flush().ok();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    let result = run(&args).await;

    spinner.abort();

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
