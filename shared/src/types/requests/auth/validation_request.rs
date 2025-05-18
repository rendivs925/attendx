#[derive(Default)]
pub struct ValidationRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
