use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateOrganizationRequest {
    pub email: String,
    pub name: String,
    pub logo_url: String,
}
