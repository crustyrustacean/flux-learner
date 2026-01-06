// src/client.rs

// dependencies
use crate::domain::OpenRouterResponse;
use crate::error::AppError;
#[cfg(feature = "vision")]
use base64::{Engine, engine::general_purpose::STANDARD};
use rama::http::BodyExtractExt;
use rama::http::headers::ContentType;
use rama::http::service::client::HttpClientExt;
use rama::service::{BoxService, Service};
use rama::{
    error::OpaqueError,
    http::{Request, Response, client::EasyHttpWebClient},
};
#[cfg(feature = "vision")]
use std::path::Path;

pub struct OpenRouterClient {
    http_client: BoxService<Request, Response, OpaqueError>,
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
            http_client: EasyHttpWebClient::default().boxed(),
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
        let body = format!(
            r#"{{
        "model": {},
        "messages": [
            {{
                "role": "system",
                "content": {}
            }},
            {{
                "role": "user",
                "content": {}
            }}
        ]
    }}"#,
            serde_json::to_string(&model).unwrap(),
            serde_json::to_string(&system_prompt).unwrap(),
            serde_json::to_string(&user_prompt).unwrap()
        );

        let response = self
            .http_client
                .post(&self.base_url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .typed_header(ContentType::json())
                .json(&body)
                .send()
                .await?;

        let chat_response = response.try_into_json::<OpenRouterResponse>().await?;

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

        let body = format!(
            r#"{{
        "model": {},
        "messages": [
            {{
                "role": "system",
                "content": {}
            }},
            {{
                "role": "user",
                "content": [
                    {{ "type": "text", "text": {} }},
                    {{ "type": "image_url", "image_url": {{ "url": {} }} }}
                ]
            }}
        ]
    }}"#,
            serde_json::to_string(&model).unwrap(),
            serde_json::to_string(&system_prompt).unwrap(),
            serde_json::to_string(&user_prompt).unwrap(),
            serde_json::to_string(&data_uri).unwrap()
        );

        let response = self
            .http_client
                .post(&self.base_url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .typed_header(ContentType::json())
                .json(&body)
                .send()
                .await?;

        println!("Status: {}", response.status());
        let chat_response = response.try_into_json::<OpenRouterResponse>().await?;

        Ok(chat_response)
    }
}
