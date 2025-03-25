use std::collections::HashMap;

use derive_getters::{Dissolve, Getters};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct RealmID(usize);

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct RealmTheme {
    logo: String,
    accent_color: String,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct RealmAdminCapability {
    discussion: bool,
    chat: bool,
    workspaces: bool,
    lessons: bool,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct RealmSettings {
    course_inactive_on_lti_creation: bool,
    allow_course_creation: bool,
    lti_and_course_creation: bool,
    discuss_shared_category: String,
    theme: RealmTheme,
    sourced_id_as_unique_identifier: bool,
    allow_chat: bool,
    force_name_update: bool,
    realm_admin_capability: RealmAdminCapability,
    allow_lessons_and_workspaces_enable: bool,
    // lexical_access: Option<_>,
}

#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct Realm {
    id: RealmID,
    name: String,
    #[serde(rename = "type")]
    type_: String,
    domain: String,
    associated_domains: String,
    features: (),
    settings: RealmSettings,
    affiliate_realm_id: Option<RealmID>,
}
