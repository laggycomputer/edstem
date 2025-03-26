use derive_getters::{Dissolve, Getters};
use serde::Deserialize;

use super::{
    lab::{Lab, LabID}, realm::RealmID, thread::CourseThreads, user::{digest_interval_deserialize, UserID}
};

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Eq, Dissolve)]
pub struct CourseID(u64);

impl Into<u64> for CourseID {
    fn into(self) -> u64 {
        self.0
    }
}

impl CourseID {
    pub async fn get_threads(&self, client: &crate::Client) -> crate::Result<CourseThreads> {
        client.get_course_threads(self.clone()).await
    }

    pub async fn get_thread_by_number(&self, client: &crate::Client, thread_number: u64) -> crate::Result<Thread> {
        client.get_thread_by_number(self, thread_number).await
    }
}

/// overrides of global settings for a single course
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseRoleSettings {
    #[serde(deserialize_with = "digest_interval_deserialize")]
    digest_interval: Option<u64>,
    email_announcements: Option<bool>,
}

/// The role of a [`crate::model::user::User`] in a [`Course`]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum Role {
    #[serde(rename = "student")]
    Student,
    #[serde(rename = "mentor")]
    Mentor,
    #[serde(rename = "tutor")]
    Tutor,
    #[serde(rename = "staff")]
    Staff,
    #[serde(rename = "admin")]
    Admin,
    Other(String),
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseRole {
    user_id: UserID,
    course_id: CourseID,
    lab_id: Option<LabID>,
    role: Role,
    // tutorial: Option<_>,
    digest: bool,
    settings: CourseRoleSettings,
    created_at: String,
    deleted_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseFeatures {
    analytics: bool,
    discussion: bool,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct Category {
    name: String,
    subcategories: Vec<Box<Category>>,
    thread_template: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseDiscussionSettings {
    /// can private threads be created?
    private: bool,
    private_threads_only: bool,
    /// speculation: are replies to anonymously posted threads made by the same author marked
    /// anonymous?
    anonymous_comments: bool,
    anonymous_comments_override: bool,
    /// speculation: can users start threads anonymously?
    anonymous: bool,
    /// can staff members see the true identity of anonymous posters?
    anonymous_to_staff: bool,
    threads_require_approval: bool,
    unread_indicator_hidden: bool,
    deleted: bool,
    categories: Vec<Category>,
    thread_templates_enabled: bool,
    category_unselected: bool,
    snippet_langauges: Vec<String>,
    /// when users add code snippets to their posts, which language is default?
    default_snippet_language: String,
    rejection_comment_template: Option<String>,
    // bot_source: Option<_>,
    bot_enabled: bool,
    bot_enabled_v2: bool,
    bot_name: String,
    bot_avatar: String,
    full_announcement_emails: bool,
    no_digests: bool,
    #[serde(deserialize_with = "digest_interval_deserialize")]
    digest_interval: Option<u64>,
    saved_replies_enabled: bool,
    saved_replies: Vec<String>,
    sortable_feed: bool,
    default_feed_sort: String,
    /// speculation: whether to show number of likes and other stats on threads
    thread_numbers: bool,
    /// speculation: whether to show number of likes and other stats on thread comments
    comment_numbers: bool,
    tutorial_badge_visible_to_all: bool,
    tutorial_badge_visible_anon: bool,
    /// staff can mark a course as read only e.g. once the term is over
    readonly: bool,
    /// if false, only the two most recently posted pinned threads show up
    show_all_pinned_threads: bool,
    comment_endorsements: bool,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseChatSettings {
    student_dm_student: bool,
    student_dm_staff: bool,
    channels_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseLessonSettings {
    quiz_question_auto_submit: bool,
    karel_slide_enabled: bool,
    workspace_partition_slide_enabled: bool,
    autoplay_videos: bool,
    hide_video_download: bool,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseWorkspaceSettingsInner {
    rstudio_layout: String,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseWorkspaceSettings {
    default_type: String,
    student_creation_disabled: bool,
    remote_desktop: bool,
    remote_app: bool,
    saturn_override: bool,
    saturn_default_kernel: String,
    disable_student_workspace_upload: bool,
    extra_paths: bool,
    // env: Option<_>,
    settings: CourseWorkspaceSettingsInner,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseCodeEditorSettings {
    // show_invisibles: Option<_>,
    // detect_indentation: Option<_>,
    // soft_tabs: Option<_>,
    // tab_size: Option<_>,
    // autocomplete: Option<_>,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseTheme {
    logo: String,
    background: String,
    foreground: String,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseRoleLabels {
    student: String,
    mentor: String,
    tutor: String,
    staff: String,
    admin: String,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseSettings {
    default_page: String,
    user_lab_enrollment: bool,
    lab_user_agent_regex: String,
    lockdown_user_agent_regex: String,
    access_codes_enabled: bool,
    access_codes_public: bool,
    setup_status: String,
    discussion: CourseDiscussionSettings,
    chat: CourseChatSettings,
    lesson: CourseLessonSettings,
    workspace: CourseWorkspaceSettings,
    challenge_workspace: CourseWorkspaceSettings,
    code_editor: CourseCodeEditorSettings,
    theme: CourseTheme,
    role_labels: CourseRoleLabels,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct Course {
    id: CourseID,
    realm_id: RealmID,
    code: String,
    name: String,
    year: String,
    session: String,
    status: String,
    features: CourseFeatures,
    settings: CourseSettings,
    created_at: String,
    is_lab_regex_active: bool,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct SelfUserCourse {
    course: Course,
    role: CourseRole,
    lab: Option<Lab>,
    /// last time this course had any activity
    last_active: String,
}
