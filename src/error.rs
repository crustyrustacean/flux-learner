// src/error.rs

// dependencies
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Environment variable error: {0}")]
    Env(#[from] env::VarError),

    #[error("Failed to load .env file: {0}")]
    DotEnv(#[from] dotenvy::Error),

    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
}
