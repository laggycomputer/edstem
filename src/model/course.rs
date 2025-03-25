use derive_getters::{Dissolve, Getters};
use serde::Deserialize;

use super::{lab::{Lab, LabID}, user::{digest_interval_deserialize, UserID}};

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Eq, Dissolve)]
pub struct CourseID(u64);

/// overrides of global settings for a single course
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseRoleSettings {
    #[serde(deserialize_with = "digest_interval_deserialize")]
    digest_interval: Option<u64>,
    email_announcements: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct CourseRole {
    user_id: UserID,
    course_id: CourseID,
    lab_id: Option<LabID>,
    role: String,
    // tutorial: Option<_>,
    digest: bool,
    settings: CourseRoleSettings,
    /// ISO 8601
    created_at: String,
    deleted_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
// TODO
pub struct Course {}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct SelfUserCourse {
    course: Course,
    role: CourseRole,
    lab: Option<Lab>,
    /// ISO 8601
    last_active: String,
}

