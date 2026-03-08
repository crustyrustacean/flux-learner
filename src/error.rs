// src/error.rs

// dependencies
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("HTTP client failed: {0}")]
    Opaque(#[from] rama::error::OpaqueError),

    #[error("Environment variable error: {0}")]
    Env(#[from] env::VarError),

    #[error("Failed to load .env file: {0}")]
    DotEnv(#[from] dotenvy::Error),

    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("File walk error: {0}")]
    Ignore(String),

    #[error("No files found: {0}")]
    EmptySource(String),

    #[error("No input specified — provide --source, --dir, or --manifest")]
    NoInput,
}
