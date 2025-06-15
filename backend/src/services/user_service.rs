use crate::{
    repositories::user_repository::UserRepository,
    utils::auth_utils::{generate_jwt, hash_password, verify_password},
};
use shared::{
    models::user_model::User,
    types::{
        requests::{
            auth::register_request::RegisterRequest, user::update_user_request::UpdateUserRequest,
        },
        responses::user_response::UserResponse,
    },
    utils::locale_utils::Namespace,
};

use shared::prelude::*;
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
            UserServiceError::DbError(_) => {
                messages.get_message(Namespace::Auth, "register.db_error")
            }
            UserServiceError::JwtGenerationError(_) => {
                messages.get_message(Namespace::Auth, "auth.jwt_generation_failed")
            }
            UserServiceError::PasswordHashingError(_) => {
                messages.get_message(Namespace::Auth, "auth.password_hashing_failed")
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

    pub async fn authenticate_user(
        &self,
        email: &str,
        password: &str,
    ) -> Result<(UserResponse, String), UserServiceError> {
        let user = self
            .user_repository
            .find_user("email", email)
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))?
            .ok_or(UserServiceError::NotFound)?;

        if !verify_password(password, &user.password) {
            return Err(UserServiceError::InvalidCredentials);
        }

        let token = generate_jwt(&user.name, &user.email)
            .map_err(|e| UserServiceError::JwtGenerationError(e.to_string()))?;

        Ok((UserResponse::from(user), token))
    }

    pub async fn register_user(
        &self,
        new_user: RegisterRequest,
    ) -> Result<UserResponse, UserServiceError> {
        let existing_user = self
            .user_repository
            .find_user("email", &new_user.email)
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))?;

        if existing_user.is_some() {
            return Err(UserServiceError::DuplicateEmail);
        }

        let hashed_password = hash_password(&new_user.password)
            .map_err(|e| UserServiceError::PasswordHashingError(e.to_string()))?;

        let user = User {
            name: new_user.name,
            email: new_user.email.clone(),
            password: hashed_password,
            ..Default::default()
        };

        self.user_repository
            .register_user(&user)
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))?;

        Ok(UserResponse::from(user))
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
            .find_user("email", email)
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))?;

        Ok(user.map(UserResponse::from))
    }

    pub async fn update_user(
        &self,
        email: &str,
        user: UpdateUserRequest,
    ) -> Result<UpdateUserRequest, UserServiceError> {
        self.user_repository
            .update_user(email, user)
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))
    }

    pub async fn delete_user(&self, email: &str) -> Result<(), UserServiceError> {
        self.user_repository
            .delete_user(email)
            .await
            .map_err(|e| UserServiceError::DbError(e.to_string()))
    }
}
