use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::models::organization::organization_limit::OrganizationLimits;
use crate::types::models::user::subscription::SubscriptionPlan;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Organization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub owner_id: ObjectId,
    pub password: String,
    pub logo_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub limits: OrganizationLimits,
}

impl Default for Organization {
    fn default() -> Self {
        let now = Utc::now();

        let default_limits = OrganizationLimits::from_plan(&SubscriptionPlan::Free);

        Self {
            _id: Some(ObjectId::new()),
            name: String::default(),
            email: String::default(),
            owner_id: Default::default(),
            password: String::default(),
            logo_url: Default::default(),
            created_at: now,
            updated_at: now,
            limits: default_limits,
        }
    }
}
