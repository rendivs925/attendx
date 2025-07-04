use async_graphql::{ID, SimpleObject};
use serde::{Deserialize, Serialize};
use shared::types::{
    models::user::{role::Role, user_status::UserStatus},
    responses::user_response::UserResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct UserObject {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub organization_id: ID,
    pub role: Role,
    pub status: UserStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl From<shared::models::user_model::User> for UserObject {
    fn from(user: shared::models::user_model::User) -> Self {
        Self {
            id: ID::from(user.id.to_string()),
            name: user.name,
            email: user.email,
            organization_id: ID::from(user.organization_id.to_string()),
            role: user.role,
            status: user.status,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        }
    }
}

impl From<UserResponse> for UserObject {
    fn from(u: UserResponse) -> Self {
        Self {
            id: u.id.to_string().into(),
            name: u.name,
            email: u.email,
            organization_id: u.organization_id.to_string().into(),
            role: u.role,
            status: u.status,
            created_at: u.created_at.to_rfc3339(),
            updated_at: u.updated_at.to_rfc3339(),
        }
    }
}
