use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum GlobalRole {
    User,
    Developer,
}

impl Default for GlobalRole {
    fn default() -> Self {
        Self::User
    }
}
