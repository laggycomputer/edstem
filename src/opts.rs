//! Options for API requests.

#[cfg(feature = "serde")]
use serde::Deserialize;

use serde::Serialize;

/// How to sort responses as part of [`GetCourseThreadsOptions`].
/// All unit variants are sort keys with known meaning.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum GetCourseThreadsSortKey {
    /// Newest threads first.
    New,
    /// Another sort key not known to this crate.
    Other(String),
}

impl ToString for GetCourseThreadsSortKey {
    fn to_string(&self) -> String {
        match self {
            Self::New => String::from("new"),
            Self::Other(inner) => inner.clone(),
        }
    }
}

/// Options to [`crate::Client::get_course_threads`], centered on skip-take pagination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GetCourseThreadsOptions {
    /// The limit on the number of threads to return. Values not greater than 100 appear to be
    /// treated as 100 by Ed Discussion.
    pub limit: u64,
    /// The offset at which to begin; i.e. how many threads to skip before beginning to yield
    /// threads.
    pub offset: u64,
    /// A key by which to sort returned threads. See documentation for [`GetCourseThreadsSortKey`].
    pub sort: GetCourseThreadsSortKey,
}

impl Default for GetCourseThreadsOptions {
    fn default() -> Self {
        Self {
            limit: 20,
            offset: 0,
            sort: GetCourseThreadsSortKey::New,
        }
    }
}

impl GetCourseThreadsOptions {
    pub(crate) fn as_params(&self) -> Vec<(&str, impl Serialize)> {
        vec![
            ("limit", self.limit.to_string()),
            ("offset", self.offset.to_string()),
            ("sort", self.sort.to_string()),
        ]
    }
}
