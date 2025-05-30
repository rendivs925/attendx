use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum GlobalRole {
    User,
    Developer,
}

impl Default for GlobalRole {
    fn default() -> Self {
        Self::User
    }
}
