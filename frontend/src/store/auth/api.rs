use crate::constants::API_BASE_URL;
use gloo_net::http::Request;
use gloo_net::http::Response;
use leptos::web_sys::RequestCredentials;
use serde::{Deserialize, Serialize};

const SUPABASE_URL: &str = "https://hxyuphznpsjkixxdmkan.supabase.co";
const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Imh4eXVwaHpucHNqa2l4eGRta2FuIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTExMDk5NDIsImV4cCI6MjA2NjY4NTk0Mn0.CWRoNRxXqWezCko1AeUOZrGRfzzDcdMW_QXr0vbk-DY";

#[derive(Serialize, Clone)]
pub struct RegisterPayload {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Serialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct SupabaseAuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub user: serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
}

pub async fn send_register_request(payload: &RegisterPayload) -> Result<Response, String> {
    if payload.password != payload.password_confirmation {
        return Err("Password confirmation does not match".into());
    }

    let auth_body = serde_json::json!({
        "email": payload.email,
        "password": payload.password,
        "data": {
            "name": payload.name
        }
    });

    let auth_res = Request::post(&format!("{SUPABASE_URL}/auth/v1/signup"))
        .header("Content-Type", "application/json")
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Authorization", &format!("Bearer {SUPABASE_ANON_KEY}"))
        .body(serde_json::to_string(&auth_body).unwrap())
        .map_err(|e| format!("Failed to build auth request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Supabase signup error: {e}"))?;

    if !auth_res.ok() {
        return Err(format!(
            "Supabase error: {} ({})",
            auth_res.status(),
            auth_res.text().await.unwrap_or_default()
        ));
    }

    let auth_json = auth_res
        .json::<SupabaseAuthResponse>()
        .await
        .map_err(|e| format!("Auth parse error: {e}"))?;

    let user_body = serde_json::json!({
        "name": payload.name,
        "email": payload.email
    });

    let backend_res = Request::post(&format!("{}/auth/register", *API_BASE_URL))
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            &format!("Bearer {}", auth_json.access_token),
        )
        .credentials(RequestCredentials::Include)
        .body(serde_json::to_string(&user_body).unwrap())
        .map_err(|e| format!("Failed to build backend request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Backend error: {e}"))?;

    if !backend_res.ok() {
        return Err(format!("Backend returned status {}", backend_res.status()));
    }

    Ok(backend_res)
}

pub async fn send_login_request(payload: &LoginPayload) -> Result<Response, String> {
    let login_body = serde_json::json!({
        "email": payload.email,
        "password": payload.password
    });

    let res = Request::post(&format!("{SUPABASE_URL}/auth/v1/token?grant_type=password"))
        .header("Content-Type", "application/json")
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Authorization", &format!("Bearer {SUPABASE_ANON_KEY}"))
        .body(serde_json::to_string(&login_body).unwrap())
        .map_err(|e| format!("Failed to build login request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Connection error: {e}"))?;

    Ok(res)
}
