use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Active
    }
}
