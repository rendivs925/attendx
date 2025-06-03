use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::models::organization_member::member_role::MemberRole;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizationMember {
    pub organization_id: String,
    pub name: String,
    pub role: MemberRole,
    pub identifiers: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

impl Default for OrganizationMember {
    fn default() -> Self {
        OrganizationMember {
            organization_id: String::default(),
            name: String::new(),
            role: MemberRole::default(),
            identifiers: HashMap::new(),
            created_at: Utc::now(),
        }
    }
}
