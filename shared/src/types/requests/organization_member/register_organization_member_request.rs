use crate::types::models::organization_member::member_role::MemberRole;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct RegisterOrganizationMemberRequest {
    pub organization_id: String,
    pub name: String,
    pub role: MemberRole,
    pub identifiers: HashMap<String, String>,
}
