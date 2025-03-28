//! Options for API requests.

#[cfg(feature = "serde")]
use serde::Deserialize;

use serde::Serialize;
use strum_macros::AsRefStr;

/// How to sort responses as part of [`GetCourseThreadsOptions`].
/// All unit variants are sort keys with known meaning.
#[derive(Clone, Debug, PartialEq, Eq, AsRefStr)]
#[strum(serialize_all = "lowercase")]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[non_exhaustive]
pub enum GetCourseThreadsSortKey {
    /// Newest threads first.
    New,
}

/// A filter mode for [`GetCourseThreadsOptions`].
#[derive(Clone, Debug, PartialEq, Eq, AsRefStr)]
#[strum(serialize_all = "lowercase")]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[non_exhaustive]
pub enum GetCourseThreadsFilterKey {
    /// Return only threads not already read by the current logged in user.
    Unread,
    /// Return only threads read by the current user which also have unread replies.
    NewReplies,
    /// Return only threads of type [`ThreadType::Question`](crate::model::thread::ThreadType::Question) which have no answers/replies.
    Unanswered,
    /// Return only threads not marked resolved by staff.
    Unresolved,
    /// Return only questions with at least one answer which has been endorsed by a staff member.
    Endorsed,
    /// Return only threads which are marked as
    /// [`ThreadWatchStatus::Watching`](crate::model::thread::ThreadWatchStatus::Watching) by the
    /// current logged in user.
    Watching,
    /// Return only threads which are starred by the current logged in user.
    Starred,
    /// Return only threads which are private and visible to the current logged in user, because they
    /// are the author or course staff.
    Private,
    /// Return only publicly (course-wide) visible threads.
    Public,
    /// Return only threads initially posted by course staff.
    Staff,
    /// Return only threads initially posted by the current logged in user.
    Me,
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
    /// An optional filter key, where `None` means no filter.
    pub filter: Option<GetCourseThreadsFilterKey>,
}

impl Default for GetCourseThreadsOptions {
    fn default() -> Self {
        Self {
            limit: 20,
            offset: 0,
            sort: GetCourseThreadsSortKey::New,
            filter: None,
        }
    }
}

impl GetCourseThreadsOptions {
    pub(crate) fn as_params(&self) -> Vec<(&str, impl Serialize)> {
        let mut ret = vec![
            ("limit", self.limit.to_string()),
            ("offset", self.offset.to_string()),
            ("sort", self.sort.as_ref().to_string()),
        ];

        if let Some(ref filter) = self.filter {
            ret.push(("filter", filter.as_ref().to_string()));
        }

        ret
    }
}
