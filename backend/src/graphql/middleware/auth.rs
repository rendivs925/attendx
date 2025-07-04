use crate::constants::SUPABASE_JWT_SECRET;
use crate::graphql::error::AppError;
use crate::graphql::error::graphql_error;
use async_graphql::{Context, Result};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde_json::Value;

pub fn validate_token(token: &str) -> Result<Value, AppError> {
    let secret = SUPABASE_JWT_SECRET.as_str();
    println!("[DEBUG] Decoding token with secret: {}", secret);

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["authenticated"]);

    decode::<Value>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| {
        println!("[DEBUG] Token valid. Claims: {:?}", data.claims);
        data.claims
    })
    .map_err(|err| {
        println!("[ERROR] Failed to decode token: {}", err);
        AppError::Unauthorized
    })
}

pub fn extract_claims(ctx: &Context<'_>) -> Result<Value> {
    if let Some(token) = ctx.data_opt::<String>() {
        println!("[DEBUG] JWT token found in context: {}", token);
        return validate_token(token).map_err(graphql_error);
    }

    println!("[DEBUG] No JWT token found in context");
    Err(graphql_error(AppError::Unauthorized))
}
