use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::organization_model::Organization;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganizationResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub max_users: i32,
    pub max_attendance_logs: i32,
    pub owner_id: String,
    pub logo_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Organization> for OrganizationResponse {
    fn from(organization: Organization) -> Self {
        Self {
            id: organization.id.to_string(),
            name: organization.name,
            max_attendance_logs: organization.max_attendance_logs,
            max_users: organization.max_users,
            email: organization.email,
            owner_id: organization.owner_id.to_string(),
            logo_url: organization.logo_url,
            created_at: organization.created_at,
            updated_at: organization.updated_at,
        }
    }
}
