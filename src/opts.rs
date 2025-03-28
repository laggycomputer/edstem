use serde::{Deserialize, Serialize};

/// Options to [`crate::Client::get_course_threads`]. Note that `limit` appears to be limited to at
/// most 100 by Ed Discussion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GetCourseThreadsOptions {
    pub limit: u64,
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
