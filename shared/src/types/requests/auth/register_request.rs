use async_graphql::InputObject;
use serde::Deserialize;

#[derive(Debug, InputObject, Default, Deserialize)]
pub struct RegisterRequest {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
}
