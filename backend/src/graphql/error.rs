use async_graphql::{Error as GQLError, ErrorExtensions};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

pub fn graphql_error(err: AppError) -> GQLError {
    let code = match &err {
        AppError::Unauthorized => "UNAUTHORIZED",
        AppError::Validation(_) => "VALIDATION",
        AppError::NotFound(_) => "NOT_FOUND",
        AppError::Conflict(_) => "CONFLICT",
        AppError::Internal(_) => "INTERNAL",
    };

    GQLError::new(err.to_string()).extend_with(|_, e| {
        e.set("code", code);
    })
}
