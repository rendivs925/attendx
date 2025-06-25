use crate::{
    models::organization_member_model::OrganizationMember,
    types::models::organization_member::member_role::MemberRole,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OrganizationMemberResponse {
    pub organization_id: String,
    pub name: String,
    pub role: MemberRole,
    pub identifiers: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

impl From<OrganizationMember> for OrganizationMemberResponse {
    fn from(member: OrganizationMember) -> Self {
        OrganizationMemberResponse {
            organization_id: member.organization_id,
            name: member.name,
            role: member.role,
            identifiers: member.identifiers,
            created_at: member.created_at,
        }
    }
}
