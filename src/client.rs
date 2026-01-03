// src/client.rs

// dependencies
use crate::domain::OpenRouterResponse;
use crate::error::AppError;
use reqwest::Client;
use serde_json::json;

// constants
const OPENROUTER_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

pub struct OpenRouterClient {
    http_client: Client,
    api_key: String,
}

impl OpenRouterClient {
    pub fn new(api_key: String) -> Self {
        let http_client = Client::new();

        Self {
            http_client,
            api_key,
        }
    }

    pub async fn chat(
        &self,
        model: &str,
        user_prompt: &str,
        system_prompt: &str,
    ) -> Result<OpenRouterResponse, AppError> {
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

        let response = self
            .http_client
            .post(OPENROUTER_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        let chat_response = response.json::<OpenRouterResponse>().await?;

        Ok(chat_response)
    }
}
