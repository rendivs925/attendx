use serde::{Deserialize, Serialize};
#[cfg(feature = "backend")]
use sqlx::FromRow;

#[cfg_attr(feature = "backend", derive(FromRow))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}
