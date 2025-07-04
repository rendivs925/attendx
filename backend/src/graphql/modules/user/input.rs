use async_graphql::InputObject;
use shared::types::requests::auth::register_request::RegisterRequest;

#[derive(InputObject)]
pub struct RegisterInput {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
}

impl RegisterInput {
    pub fn to_register_request(self) -> RegisterRequest {
        RegisterRequest {
            id: self.id,
            name: self.name,
            email: self.email,
        }
    }
}
