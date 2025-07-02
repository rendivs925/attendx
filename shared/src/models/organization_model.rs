use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "backend")]
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(FromRow))]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub owner_id: Uuid,
    pub logo_url: String,
    pub max_users: i32,
    pub max_attendance_logs: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Organization {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            email: String::new(),
            owner_id: Uuid::new_v4(),
            logo_url: String::new(),
            max_users: 0,
            max_attendance_logs: 0,
            created_at: now,
            updated_at: now,
        }
    }
}
