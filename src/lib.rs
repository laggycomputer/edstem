//! API for Ed Discussion (https://edstem.org).
//! Request an API key at https://edstem.org/us/settings/api-tokens.
//!
//! ## notes
//!
//! all `avatar` fields are an ID; the actual image is accessible at https://static.us.edusercontent.com/avatars/{id}
#![deny(missing_docs)]

use model::user::SelfUser;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

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
    base_url: String,
    token: String,
    user_agent: String,
}

type EmptyParams = &'static [((), ())];

impl Client {
    async fn request<T>(&self, request: RequestBuilder) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let built = request
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", &self.user_agent)
            .build()?;

        let response = self.http.execute(built).await?;

        Ok(response.json().await?)
    }

    async fn get<T>(
        &self,
        endpoint: &str,
        parameters: Option<&[(impl Serialize, impl Serialize)]>,
    ) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut builder = self.http.get(format!("{}{}", self.base_url, endpoint));

        if let Some(params) = parameters {
            builder = builder.query(params);
        };

        Ok(self.request(builder).await?)
    }

    /// Get the [`SelfUser`] representing the user making API requests.
    pub async fn get_self_user(&self) -> Result<SelfUser> {
        Ok(self.get("/api/user", None::<EmptyParams>).await?)
    }

    /// Get the [`CourseThreads`] pertaining to a course.
    pub async fn get_course_threads(&self, id: impl Into<u64>) -> Result<CourseThreads> {
        let endpoint = format!("/api/courses/{}/threads", id.into());
        Ok(self.get(&*endpoint, None::<EmptyParams>).await?)
    }
}
