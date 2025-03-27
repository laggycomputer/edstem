use derive_getters::{Dissolve, Getters};
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Eq, Dissolve)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LabID(u64);

// i have no idea what this does
#[derive(Clone, Debug, Deserialize, Getters, Dissolve)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Lab {}
