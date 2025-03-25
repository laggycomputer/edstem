// #![deny(missing_docs))

use reqwest::RequestBuilder;
use serde_json::Value;

pub mod model;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("error deserializing json: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Clone, Debug)]
pub struct Client {
    http: reqwest::Client,
    endpoint: String,
    token: String,
    user_agent: String,
}

impl Client {
    async fn request<T>(&self, request: RequestBuilder) -> Result<Value>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let built = request
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", &self.user_agent)
            .build()?;

        let response = self.http.execute(built).await?;

        Ok(response.json().await?)
    }
}
