use crate::graphql::error::AppError;
use crate::repositories::user_repository::UserRepository;
use shared::{
    models::user_model::User,
    types::requests::{
        auth::register_request::RegisterRequest, user::update_user_request::UpdateUserRequest,
    },
    types::responses::user_response::UserResponse,
};
use std::sync::Arc;

pub struct UserService {
    pub user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn register_user(&self, new_user: RegisterRequest) -> Result<UserResponse, AppError> {
        let exists = self
            .user_repository
            .find_user(&new_user.email)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if exists.is_some() {
            return Err(AppError::Conflict("Email already registered".into()));
        }

        let user = User {
            name: new_user.name,
            email: new_user.email.clone(),
            ..Default::default()
        };

        let saved = self
            .user_repository
            .register_user(&user)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(UserResponse::from(saved))
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, AppError> {
        let users = self
            .user_repository
            .get_all_users()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    pub async fn get_user(&self, email: &str) -> Result<Option<UserResponse>, AppError> {
        let user = self
            .user_repository
            .find_user(email)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(user.map(UserResponse::from))
    }

    pub async fn update_user(
        &self,
        email: &str,
        update: UpdateUserRequest,
    ) -> Result<UserResponse, AppError> {
        let updated = self
            .user_repository
            .update_user(email, update)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(UserResponse::from(updated))
    }

    pub async fn delete_user(&self, email: &str) -> Result<(), AppError> {
        self.user_repository
            .delete_user(email)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))
    }
}
