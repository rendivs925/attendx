use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
}
