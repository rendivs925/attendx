use crate::types::models::organization_member::member_role::MemberRole;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct UpdateOrganizationMemberRequest {
    pub organization_id: String,
    pub name: Option<String>,
    pub role: Option<MemberRole>,
    pub identifiers: Option<HashMap<String, String>>,
}
