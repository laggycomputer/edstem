//! Options for API requests.

#[cfg(feature = "serde")]
use serde::Deserialize;

use serde::Serialize;

/// Options to [`crate::Client::get_course_threads`], centered on skip-take pagination.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GetCourseThreadsOptions {
    /// The limit on the number of threads to return. Values not greater than 100 appear to be
    /// treated as 100 by Ed Discussion.
    pub limit: u64,
    /// The offset at which to begin; i.e. how many threads to skip before beginning to yield
    /// threads.
    pub offset: u64,
    // not sure what this does
    // pub sort: String,
}

impl Default for GetCourseThreadsOptions {
    fn default() -> Self {
        Self {
            limit: 20,
            offset: 0,
            // sort: String::from("new"),
        }
    }
}

impl GetCourseThreadsOptions {
    pub(crate) fn as_params(&self) -> Vec<(&str, &impl Serialize)> {
        vec![
            ("limit", &self.limit),
            ("offset", &self.offset),
            // ("sort", &*self.sort),
        ]
    }
}
