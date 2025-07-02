use crate::repositories::user_repository::UserRepository;
use crate::utils::auth_utils::{generate_jwt, verify_password};
use shared::utils::locale_utils::MessageLookup;
use shared::{
    models::user_model::User,
    types::requests::{
        auth::register_request::RegisterRequest, user::update_user_request::UpdateUserRequest,
    },
    types::responses::user_response::UserResponse,
    utils::locale_utils::Namespace,
};
use std::fmt;
use std::sync::Arc;

#[derive(Debug)]
pub enum UserServiceError {
    NotFound,
    InvalidCredentials,
    DuplicateEmail,
    DbError(String),
    JwtGenerationError(String),
    PasswordHashingError(String),
}

impl UserServiceError {
    pub fn to_message(&self, messages: &dyn MessageLookup) -> String {
        match self {
            UserServiceError::NotFound => messages.get_message(Namespace::User, "fetch.not_found"),
            UserServiceError::InvalidCredentials => {
                messages.get_message(Namespace::Auth, "login.invalid_credentials")
            }
            UserServiceError::DuplicateEmail => {
                messages.get_message(Namespace::Auth, "register.duplicate")
            }
            UserServiceError::DbError(_) => messages.get_message(Namespace::Common, "db_error"),
            UserServiceError::JwtGenerationError(_) => {
                messages.get_message(Namespace::Auth, "auth.jwt_generation_failed")
            }
            UserServiceError::PasswordHashingError(_) => {
                messages.get_message(Namespace::Auth, "auth.password_hashing_failed")
            }
        }
    }
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserServiceError::NotFound => write!(f, "User not found"),
            UserServiceError::InvalidCredentials => write!(f, "Invalid credentials"),
            UserServiceError::DuplicateEmail => write!(f, "Duplicate email"),
            UserServiceError::DbError(msg) => write!(f, "Database error: {}", msg),
            UserServiceError::JwtGenerationError(msg) => write!(f, "JWT generation error: {}", msg),
            UserServiceError::PasswordHashingError(msg) => {
                write!(f, "Password hashing error: {}", msg)
            }
        }
    }
}

pub struct UserService {
    pub user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn register_user(
        &self,
        new_user: RegisterRequest,
    ) -> Result<UserResponse, UserServiceError> {
        let exists = self
            .user_repository
            .find_user(&new_user.email)
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))?;

        if exists.is_some() {
            return Err(UserServiceError::DuplicateEmail);
        }

        let user = User {
            name: new_user.name,
            email: new_user.email.clone(),
            ..Default::default()
        };

        let saved_user = self
            .user_repository
            .register_user(&user)
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))?;

        Ok(UserResponse::from(saved_user))
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, UserServiceError> {
        let users = self
            .user_repository
            .get_all_users()
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))?;

        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    pub async fn get_user(&self, email: &str) -> Result<Option<UserResponse>, UserServiceError> {
        let user = self
            .user_repository
            .find_user(email)
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))?;

        Ok(user.map(UserResponse::from))
    }

    pub async fn update_user(
        &self,
        email: &str,
        update: UpdateUserRequest,
    ) -> Result<UserResponse, UserServiceError> {
        let updated_user = self
            .user_repository
            .update_user(email, update)
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))?;

        Ok(UserResponse::from(updated_user))
    }

    pub async fn delete_user(&self, email: &str) -> Result<(), UserServiceError> {
        self.user_repository
            .delete_user(email)
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))
    }
}
