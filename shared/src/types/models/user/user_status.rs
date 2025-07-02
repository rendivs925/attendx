use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[cfg(feature = "backend")]
use sqlx::Type;

#[cfg_attr(feature = "backend", derive(Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "user_status", rename_all = "lowercase")
)]
#[derive(Debug, Display, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
