use derive_getters::{Dissolve, Getters};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Eq, Dissolve)]
pub struct LabID(u64);

// i have no idea what this does
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
pub struct Lab {}


