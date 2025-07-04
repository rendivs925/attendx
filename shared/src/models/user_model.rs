use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "backend")]
use sqlx::FromRow;

use crate::types::models::user::{role::Role, user_status::UserStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(FromRow))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub organization_id: Uuid,
    pub role: Role,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            email: String::new(),
            organization_id: Uuid::new_v4(),
            role: Role::default(),
            status: UserStatus::default(),
            created_at: now,
            updated_at: now,
        }
    }
}
