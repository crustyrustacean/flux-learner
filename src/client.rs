// src/client.rs

// dependencies
use crate::domain::OpenRouterResponse;
use crate::error::AppError;
#[cfg(feature = "vision")]
use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::Client;
use serde_json::json;
#[cfg(feature = "vision")]
use std::path::Path;

pub struct OpenRouterClient {
    http_client: Client,
    api_key: String,
    base_url: String,
}

impl OpenRouterClient {
    pub fn new(api_key: String) -> Self {
        Self::with_base_url(
            api_key,
            "https://openrouter.ai/api/v1/chat/completions".to_string(),
        )
    }

    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            api_key,
            base_url,
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
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        let chat_response = response.json::<OpenRouterResponse>().await?;

        Ok(chat_response)
    }

    #[cfg(feature = "vision")]
    pub async fn chat_with_image(
        &self,
        model: &str,
        user_prompt: &str,
        system_prompt: &str,
        image_path: impl AsRef<Path>,
    ) -> Result<OpenRouterResponse, AppError> {
        let image = std::fs::read(image_path)?;
        let image_b64 = STANDARD.encode(&image);
        let data_uri = format!("data:image/jpeg;base64,{}", image_b64);

        println!("Image bytes: {}", image.len());
        println!("Base64 length: {}", image_b64.len());
        println!("Data URI starts with: {}", &data_uri[..50]);

        let body = json! {{
                  "model": model,
                  "messages": [
                      {
                          "role": "system",
                          "content": system_prompt
                      },
                      {
                          "role": "user",
                          "content": [
          { "type": "text", "text": user_prompt },
          { "type": "image_url", "image_url": { "url": data_uri } }
        ]
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
