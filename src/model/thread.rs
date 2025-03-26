use derive_getters::{Dissolve, Getters};
use serde::{Deserialize, Deserializer};

use super::{course::CourseID, user::{ThreadParticipant, UserID}};

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Eq, Dissolve)]
pub struct ReplyID(u64);

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReplyType {
    Comment,
    Answer,
}

/// a reply to a thread
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct Reply {
    id: ReplyID,
    user_id: UserID,
    course_id: CourseID,
    thread_id: ThreadID,
    // speculation on what this ID means
    original_id: Option<ThreadID>,
    parent_id: Option<ReplyID>,
    editor_id: Option<UserID>,
    number: u64,
    #[serde(rename = "type")]
    type_: ReplyType,
    // is this ever not "normal"?
    kind: String,
    content: String,
    document: String,
    flag_count: u64,
    vote_count: u64,
    is_endorsed: bool,
    is_anonymous: bool,
    is_privte: bool,
    is_resolved: bool,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
    #[serde(deserialize_with = "anonymous_id_deserialize")]
    anonymous_id: Option<u64>,
    vote: u64,
    comments: Vec<Reply>,
}


#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Eq, Dissolve)]
pub struct ThreadID(u64);

pub(crate) fn anonymous_id_deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = u64::deserialize(deserializer)?;
    Ok(match value {
        0 => None,
        other => Some(other),
    })
}

/// the type of a thread
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum ThreadType {
    Question,
    Announcement,
    Post,
}

/// the ways in which a user may watch a thread
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreadWatchStatus {
    /// "Never be notified"
    Ignoring,
    /// "Be notified of direct replies only"
    NotWatching,
    /// "Be notified of all activity in this thread"
    Watching,
}

impl<'de> Deserialize<'de> for ThreadWatchStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<bool>::deserialize(deserializer)?;
        Ok(match opt {
            None => Self::NotWatching,
            Some(false) => Self::Ignoring,
            Some(true) => Self::Watching,
        })
    }
}

/// threads as they appear when fetching a course
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct PartialThread {
    id: ThreadID,
    user_id: UserID,
    course_id: CourseID,
    original_id: Option<ThreadID>,
    editor_id: Option<UserID>,
    /// ID of the accepted answer, if it exists
    accepted_id: Option<ReplyID>,
    /// if this thread was marked a duplicate of thread X, the ID of thread X
    duplicate_id: Option<ThreadID>,
    /// the user-facing number of this thread within the course
    number: u64,
    #[serde(rename = "type")]
    type_: ThreadType,
    title: String,
    content: String,
    document: String,
    /// potentially empty string name of a category
    category: String,
    /// potentially empty string name of a category
    subcategory: String,
    /// potentially empty string name of a category
    subsubcategory: String,
    flag_count: u64,
    star_count: u64,
    view_count: u64,
    unique_view_count: u64,
    vote_count: u64,
    reply_count: u64,
    unresolved_count: u64,
    is_locked: bool,
    is_pinned: bool,
    is_private: bool,
    is_endorsed: bool,
    is_answered: bool,
    is_student_answered: bool,
    is_staff_answered: bool,
    is_archived: bool,
    is_anonymous: bool,
    is_megathread: bool,
    anonymous_comments: bool,
    // unsure what besides "approved" is possible
    approved_status: String,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
    pinned_at: Option<String>,
    /// if this was posted anonymously, a new proxy ID which represents the anonymous poster
    #[serde(deserialize_with = "anonymous_id_deserialize")]
    anonymous_id: Option<u64>,
    vote: u64,
    /// whether this thread has been seen by the requesting user
    is_seen: bool,
    /// whether this thread has been starred by the requesting user
    is_starred: bool,
    /// how this thread is watched by the requesting user
    is_watched: ThreadWatchStatus,
    /// the last time the requesting user glanced (speculation: saw in the feed) this thread, if ever
    glanced_at: Option<String>,
    /// number of new replies since the requesting user last viewed this thread
    new_reply_count: u64,
    /// if this thread was marked a duplicate of thread X, the title of thread X
    duplicate_title: Option<String>,
    user: Option<ThreadParticipant>,
}

/// GET /api/courses/:id/threads
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseThreads {
    sort_key: String,
    threads: Vec<PartialThread>,
    users: Vec<ThreadParticipant>,
}

impl CourseThreads {
    pub async fn get(client: &crate::Client, course_id: &CourseID) -> crate::Result<Self> {
        client.get_course_threads(course_id.clone()).await
    }
}

/// Data from a thread when requested by ID
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct Thread {
    id: ThreadID,
    user_id: UserID,
    course_id: CourseID,
    original_id: Option<ThreadID>,
    editor_id: Option<UserID>,
    accepted_id: Option<ReplyID>,
    duplicate_id: Option<ThreadID>,
    number: u64,
    #[serde(rename = "type")]
    type_: ThreadType,
    title: String,
    content: String,
    document: String,
    category: String,
    subcategory: String,
    subsubcategory: String,
    flag_count: u64,
    star_count: u64,
    view_count: u64,
    unique_view_count: u64,
    vote_count: u64,
    reply_count: u64,
    unresolved_count: u64,
    is_locked: bool,
    is_pinned: bool,
    is_private: bool,
    is_endorsed: bool,
    is_student_answered: bool,
    is_staff_answered: bool,
    is_archived: bool,
    is_anonymous: bool,
    is_megathread: bool,
    anonymous_comments: bool,
    approved_status: String,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
    pinned_at: Option<String>,
    #[serde(deserialize_with = "anonymous_id_deserialize")]
    anonymous_id: Option<u64>,
    vote: u64,
    is_seen: bool,
    is_starred: bool,
    is_watched: ThreadWatchStatus,
    glanced_at: String,
    new_reply_count: u64,
    duplicate_title: Option<String>,
    answers: Vec<Reply>,
    comments: Vec<Reply>,
}
