use crate::utils::locale_utils::{Messages, Namespace};

#[derive(Debug)]
pub enum CommonError {
    NotFound,
    InvalidData,
    DuplicateEmail,
    DbError(String),
    JwtGenerationError(String),
    PasswordHashingError(String),
}

impl CommonError {
    pub fn to_message(&self, messages: &Messages) -> String {
        match self {
            CommonError::NotFound => messages.get_message(Namespace::Common, "not_found"),
            CommonError::InvalidData => messages.get_message(Namespace::Common, "invalid_data"),
            CommonError::DuplicateEmail => {
                messages.get_message(Namespace::Common, "duplicate_email")
            }
            CommonError::DbError(_) => messages.get_message(Namespace::Common, "db_error"),
            CommonError::JwtGenerationError(_) => {
                messages.get_message(Namespace::Common, "jwt_generation_failed")
            }
            CommonError::PasswordHashingError(_) => {
                messages.get_message(Namespace::Common, "password_hashing_failed")
            }
        }
    }
}
