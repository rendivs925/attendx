use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Eq, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Role {
    SuperAdmin,
    Developer,
    OrgOwner,
}

impl Default for Role {
    fn default() -> Self {
        Self::OrgOwner
    }
}
