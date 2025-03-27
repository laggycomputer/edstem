use std::num::NonZeroU64;

use derive_getters::{Dissolve, Getters};
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;

use super::{
    Empty,
    course::{Role, SelfUserCourse},
    realm::{Realm, RealmID},
};

/// GET /api/user
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SelfUser {
    courses: Vec<SelfUserCourse>,
    push_key: String,
    // push_subscriptions: Vec<_>,
    realms: Vec<Realm>,
    time: String,
    user: User,
}

impl SelfUser {
    pub async fn get(client: &crate::Client) -> crate::Result<Self> {
        client.get_self_user().await
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Eq, Dissolve)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct UserID(u64);

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum ThreadListStyle {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "ultra-compact")]
    UltraCompact,
    Other(String),
}

/// The interval at which digest emails are sent, in minutes.
/// Note that despite being labelled "Instant" in the UI, such an option corresponds to an interval
/// of 1 minute.
///
/// This setting may describe a user as a whole or a specific course.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Dissolve)]
pub struct DigestInterval(Option<NonZeroU64>);

impl<'de> Deserialize<'de> for DigestInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let got = u64::deserialize(deserializer)?;
        Ok(Self(NonZeroU64::try_from(got).ok()))
    }
}

#[cfg(feature = "serde")]
impl Serialize for DigestInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.map(|o| o.get()).unwrap_or(0).serialize(serializer)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize))]
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

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct UserSettings {
    /// frequency of emails for new threads, in minutes
    digest_interval: DigestInterval,
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
    /// set to the earliest valid ISO 8601 datetime in UTC if no snooze is active
    snooze_end: String,
    // lexical_access: Option<>,
    deactivated: bool,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct User {
    id: UserID,
    role: String,
    name: String,
    email: String,
    username: Option<String>,
    avatar: Option<String>,
    features: Empty,
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

/// a user as they appear as part of a response including threads
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ThreadParticipant {
    id: UserID,
    // is this ever not "user"?
    role: String,
    name: String,
    avatar: Option<String>,
    course_role: Option<Role>,
    // tutorials: ,
}

impl Into<ThreadParticipant> for User {
    fn into(self) -> ThreadParticipant {
        ThreadParticipant {
            id: self.id,
            // TODO: really?
            role: String::from("user"),
            name: self.name,
            avatar: self.avatar,
            course_role: self.course_role,
        }
    }
}
