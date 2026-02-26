// src/firecrawl_client.rs

// dependencies
use crate::error::AppError;
use rama::http::BodyExtractExt;
use rama::http::headers::ContentType;
use rama::http::service::client::HttpClientExt;
use rama::service::{BoxService, Service};
use rama::{
    error::OpaqueError,
    http::{Request, Response, client::EasyHttpWebClient},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FirecrawlResponse {
    pub success: bool,
    pub data: FirecrawlData,
}

#[derive(Debug, Deserialize)]
pub struct FirecrawlData {
    pub markdown: String,
    pub metadata: FirecrawlMetadata,
}

#[derive(Debug, Deserialize)]
pub struct FirecrawlMetadata {
    pub title: String,
    #[serde(rename = "sourceURL")]
    pub source_url: String,
}

pub struct FireCrawlClient {
    http_client: BoxService<Request, Response, OpaqueError>,
    api_key: String,
    base_url: String,
}

impl FireCrawlClient {
    pub fn new(api_key: String) -> Self {
        Self::with_base_url(api_key, "https://api.firecrawl.dev/v2/scrape".to_string())
    }

    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        Self {
            http_client: EasyHttpWebClient::default().boxed(),
            api_key,
            base_url,
        }
    }

    pub async fn scrape(&self, url: &str) -> Result<FirecrawlResponse, AppError> {
        let body = format!(r#"{{"url": {}}}"#, serde_json::to_string(&url).unwrap());

        let response = self
            .http_client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .typed_header(ContentType::json())
            .body(body)
            .send()
            .await?;

        let scrape_response = response.try_into_json::<FirecrawlResponse>().await?;

        Ok(scrape_response)
    }
}
