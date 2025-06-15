use gloo_net::http::Request;
use leptos::web_sys::RequestCredentials;
use serde::{Deserialize, Serialize};

use crate::constants::API_BASE_URL;

#[derive(Serialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterPayload {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse<T> {
    pub data: T,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterResponse<T> {
    pub data: T,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn send_login_request(
    payload: &LoginPayload,
) -> Result<gloo_net::http::Response, String> {
    let body =
        serde_json::to_string(payload).map_err(|e| format!("Request serialization failed: {e}"))?;

    let url = format!("{}/auth/login", *API_BASE_URL);
    let request = Request::post(&url)
        .header("Content-Type", "application/json")
        .header("Accept-Language", "ja")
        .credentials(RequestCredentials::Include)
        .body(body)
        .map_err(|e| format!("Failed to build request: {e}"))?;

    request
        .send()
        .await
        .map_err(|e| format!("Connection error: {e}"))
}

pub async fn send_register_request(
    payload: &RegisterPayload,
) -> Result<gloo_net::http::Response, String> {
    let body =
        serde_json::to_string(payload).map_err(|e| format!("Request serialization failed: {e}"))?;

    let url = format!("{}/auth/register", *API_BASE_URL);
    let request = Request::post(&url)
        .header("Content-Type", "application/json")
        .header("Accept-Language", "ja")
        .credentials(RequestCredentials::Include)
        .body(body)
        .map_err(|e| format!("Failed to build request: {e}"))?;

    request
        .send()
        .await
        .map_err(|e| format!("Connection error: {e}"))
}
