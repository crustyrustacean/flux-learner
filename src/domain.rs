// src/domain.rs

// dependencies
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenRouterResponse {
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl OpenRouterResponse {
    pub fn content(&self) -> &str {
        &self.choices[0].message.content
    }

    pub fn prompt_tokens(&self) -> u32 {
        self.usage.prompt_tokens
    }

    pub fn completion_tokens(&self) -> u32 {
        self.usage.completion_tokens
    }

    pub fn total_tokens(&self) -> u32 {
        self.usage.total_tokens
    }
}
