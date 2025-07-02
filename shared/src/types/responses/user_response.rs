use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    models::user_model::User,
    types::models::user::{role::Role, user_status::UserStatus},
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub status: UserStatus,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
            status: user.status,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
