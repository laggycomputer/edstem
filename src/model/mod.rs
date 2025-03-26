//! Structs and `impl`s for the various types in the API.

use derive_getters::{Dissolve, Getters};
use serde::Deserialize;

pub(crate) mod course;
pub(crate) mod lab;
pub(crate) mod realm;
pub(crate) mod thread;
pub(crate) mod user;

/// Stand-in for maps not known to contain any fields.
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct Empty {}
