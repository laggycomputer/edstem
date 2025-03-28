//! API for Ed Discussion (https://edstem.org).
//! Request an API key at https://edstem.org/us/settings/api-tokens.
//!
//! ## notes
//!
//! all `avatar` fields are an ID; the actual image is accessible at https://static.us.edusercontent.com/avatars/{id}
//!
//! all datetime fields are timezone-qualified ISO 8601 to microsecond precision
//!
//! post bodies are written in an XML dialect, which others have figured out [here](https://github.com/smartspot2/edapi/blob/9199e1001eb04b86bb8f68d0c5f9042453cd1387/docs/api_docs.md#document-format)
//!
//! ## features
//!
//! enable `serde` to add `Serialize` impls for structs
#![deny(missing_docs)]

use model::{
    thread::{CourseThreads, ThreadResponse},
    user::SelfUser,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

pub mod model;

/// Unified error type from the crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error from underlying `reqwest`, e.g. 403 or connectivity error.
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Error from underlying `serde_json`, i.e. JSON parsing has gone wrong.
    #[error("error deserializing json: {0}")]
    Json(#[from] serde_json::Error),
}

/// Aliased [`std::result::Result`] for this crate.
pub type Result<T> = std::result::Result<T, self::Error>;

/// An API client capable of making complete requests to Ed Discussion.
#[derive(Clone, Debug)]
pub struct Client {
    http: reqwest::Client,
    base_url: String,
    token: String,
    user_agent: String,
}

type EmptyParams = &'static [((), ())];

/// Options for creation of a new [`Client`].
#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    /// A custom `reqwest::Client`, if desired.
    pub http: Option<reqwest::Client>,
    /// The base URL, if connecting to the "typical" Ed Discussion domain is not desired.
    pub base_url: Option<String>,
    /// A user agent string, if the default is not desired.
    pub user_agent: Option<String>,
}

impl Client {
    /// Construct a new client with the minimum required arguments (i.e. a token).
    pub fn new(token: &str) -> Self {
        Self::new_with_opts(token, ClientOptions::default())
    }

    /// Construct a new client with [`ClientOptions`].
    pub fn new_with_opts(token: &str, options: ClientOptions) -> Self {
        Self {
            http: options.http.unwrap_or_else(|| reqwest::Client::new()),
            base_url: options
                .base_url
                .unwrap_or(String::from("https://us.edstem.org")),
            token: String::from(token),
            user_agent: options.user_agent.unwrap_or(String::from("edstem-rust")),
        }
    }

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

    /// Get a [`Thread`] by ID.
    pub async fn get_thread(&self, id: impl Into<u64>) -> Result<ThreadResponse> {
        let endpoint = format!("/api/threads/{}", id.into());
        Ok(self.get(&*endpoint, None::<EmptyParams>).await?)
    }

    /// Get a [`Thread`] by its number in its course.
    ///
    /// This number is not the thread ID but the 1-based number visible in the UI.
    pub async fn get_thread_by_number(
        &self,
        course_id: impl Into<u64>,
        thread_number: u64,
    ) -> Result<ThreadResponse> {
        let endpoint = format!(
            "/api/courses/{}/threads/{}",
            course_id.into(),
            thread_number
        );
        Ok(self.get(&*endpoint, None::<EmptyParams>).await?)
    }
}
