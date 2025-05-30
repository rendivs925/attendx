use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct RegisterOrganizationRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub logo_url: String,
}
