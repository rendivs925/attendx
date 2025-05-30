use crate::constants::{COOKIE_NAME, JWT_SECRET_KEY};
use actix_web::cookie::time::Duration as CookieDuration;
use actix_web::cookie::{Cookie, SameSite};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use chrono::{Duration as ChronoDuration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use log::error;
use phonenumber::{country, parse};
use rand::rngs::OsRng;
use shared::types::auth::claims::Claims;
use validator::ValidationError;

pub fn generate_cookie(token: String) -> Cookie<'static> {
    Cookie::build(COOKIE_NAME.as_str(), token)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(CookieDuration::days(7))
        .finish()
}

pub fn generate_jwt(name: &str, email: &str) -> Result<String, String> {
    let secret_key = JWT_SECRET_KEY.as_bytes();
    let expiration = Utc::now() + ChronoDuration::hours(24);

    let claims = Claims {
        name: name.to_owned(),
        email: email.to_owned(),
        exp: expiration.timestamp() as usize,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret_key),
    )
    .map_err(|e| {
        error!("Error generating JWT: {:?}", e);
        format!("JWT generation failed: {}", e)
    })
}

pub fn verify_jwt(token: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|err| {
        error!("❌ Error verifying JWT: {:?}", err);
        "Error verifying JWT".to_string()
    })
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| {
            error!("❌ Error hashing password: {:?}", err);
            "Error hashing password".to_string()
        })
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(password_hash) {
        let argon2 = Argon2::default();
        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    } else {
        error!("❌ Failed to parse password hash");
        false
    }
}

pub fn validate_phone_number(phone: &str) -> Result<(), ValidationError> {
    parse(Some(country::ID), phone)
        .map(|_| ())
        .map_err(|_| ValidationError::new("invalid_phone_number"))
}
