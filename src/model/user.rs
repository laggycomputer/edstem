use std::collections::HashMap;

use derive_getters::{Dissolve, Getters};
use serde::{Deserialize, Deserializer};

use super::{course::{CourseRole, Role, SelfUserCourse}, realm::{Realm, RealmID}};

// GET /api/user
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct SelfUser {
    courses: Vec<SelfUserCourse>,
    push_key: String,
    // push_subscriptions: Vec<_>,
    realms: Vec<Realm>,
    /// appears to be ISO 8601
    time: String,
    user: User,
}

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Eq, Dissolve)]
pub struct UserID(u64);

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum ThreadListStyle {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "ultra-compact")]
    UltraCompact,
    Other(String),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum Theme {
    #[serde(rename = "os")]
    OS,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    Other(String),
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct DesktopNotificationScopes {
    announcement: bool,
    // currently no way to change this in UI
    thread: bool,
    direct_reply: bool,
    mention: bool,
    // currently no way to change this in UI
    chat: bool,
    watch: bool,
}

pub(crate) fn digest_interval_deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<u64>::deserialize(deserializer)?;
    Ok(match value {
        Some(0) => None,
        other => other,
    })
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
struct UserSettings {
    /// frequency of emails for new threads, in minutes
    ///
    /// an interval of 1 minute is labelled "instant" in the UI
    /// an interval of 0 minutes is labelled "none" in the UI and is rendered here as [`None`]
    #[serde(deserialize_with = "digest_interval_deserialize")]
    digest_interval: Option<u64>,
    /// "Thread List Style" in appearance settings
    discuss_feed_style: ThreadListStyle,
    accessible: bool,
    /// the "Language" setting in "Language & Region" e.g. `"en_us"`
    ///
    /// may be an empty string if the user has never set a locale
    locale: String,
    /// UI theme in appearance settings
    theme: Theme,
    character_key_shortcuts_disabled: bool,
    set_tz_automatically: bool,
    /// a tz database identifier
    tz: String,
    // the following are the "Notification Emails" section of Notifications page in settings
    reply_via_email: bool,
    email_announcements: bool,
    email_watched_threads: bool,
    email_thread_replies: bool,
    email_comment_replies: bool,
    email_mentions: bool,
    mention_direct_message_digest_interval: String,
    channel_digest_interval: String,
    allow_password_login: bool,
    desktop_notifications_enabled: bool,
    desktop_notifications_scopes: DesktopNotificationScopes,
    // ISO 8601; set to the earliest valid ISO 8601 datetime in UTC if no snooze is active
    snooze_end: String,
    // lexical_access: Option<>,
    deactivated: bool,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
struct User {
    id: UserID,
    role: CourseRole,
    name: String,
    email: String,
    username: Option<String>,
    avatar: String,
    features: (),
    settings: UserSettings,
    activated: bool,
    created_at: String,
    course_role: Option<Role>,
    secondary_emails: Vec<String>,
    has_password: bool,
    is_lti: bool,
    is_sso: bool,
    can_change_name: bool,
    has_pats: bool,
    realm_id: Option<RealmID>,
}
