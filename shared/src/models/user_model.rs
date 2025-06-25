use crate::types::models::user::{
    role::Role, subscription::SubscriptionPlan, user_status::UserStatus,
};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub organization_ids: HashSet<ObjectId>,
    pub owned_organizations: u32,
    pub subscription_plan: SubscriptionPlan,
    pub status: UserStatus,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            _id: Some(ObjectId::new()),
            name: String::default(),
            email: String::default(),
            password: String::default(),
            organization_ids: Default::default(),
            owned_organizations: Default::default(),
            subscription_plan: Default::default(),
            status: Default::default(),
            role: Default::default(),
            created_at: now,
            updated_at: now,
        }
    }
}
