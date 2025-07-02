use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "role", rename_all = "lowercase")
)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Superadmin,
    Developer,
    Orgowner,
}

impl Default for Role {
    fn default() -> Self {
        Self::Orgowner
    }
}
