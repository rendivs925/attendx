use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    models::organization_model::Organization,
    types::models::organization::organization_limit::OrganizationLimits,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub owner_id: ObjectId,
    pub logo_url: String,
    pub limits: OrganizationLimits,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Organization> for OrganizationResponse {
    fn from(organization: Organization) -> Self {
        Self {
            id: organization._id.map(|oid| oid.to_hex()).unwrap_or_default(),
            name: organization.name,
            email: organization.email,
            owner_id: organization.owner_id,
            logo_url: organization.logo_url,
            limits: organization.limits,
            created_at: organization.created_at,
            updated_at: organization.updated_at,
        }
    }
}
