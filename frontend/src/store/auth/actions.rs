use crate::store::auth::state::{AuthStore, ToastType};
use gloo_net::http::Request;
use leptos::prelude::{RwSignal, Set, Update};
use leptos::web_sys::RequestCredentials;
use serde::{Deserialize, Serialize};
use shared::types::responses::user_response::UserResponse;

#[derive(Serialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterPayload {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct LoginResponse {
    data: UserResponse,
    message: String,
}

#[derive(Deserialize, Debug)]
struct RegisterResponse {
    data: UserResponse,
    message: String,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    message: String,
}

fn init_auth_loading(auth: &RwSignal<AuthStore>) {
    auth.update(|auth| {
        auth.is_loading.set(true);
        auth.error_message.set(None);
        auth.toast_message.set(None);
        auth.is_toast_visible.set(false);
        auth.toast_type.set(None);
    });
}

fn show_error(auth: &RwSignal<AuthStore>, message: String) {
    auth.update(|auth| {
        auth.error_message.set(Some(message.clone()));
        auth.toast_message.set(Some(message));
        auth.toast_type.set(Some(ToastType::Error));
        auth.is_toast_visible.set(true);
        auth.is_loading.set(false);
        auth.user.set(None);
    });
}

fn show_success(auth: &RwSignal<AuthStore>, message: String, user: UserResponse) {
    auth.update(|auth| {
        auth.user.set(Some(user));
        auth.error_message.set(None);
        auth.toast_message.set(Some(message));
        auth.toast_type.set(Some(ToastType::Success));
        auth.is_toast_visible.set(true);
        auth.is_loading.set(false);
    });
}

async fn send_login_request(payload: &LoginPayload) -> Result<gloo_net::http::Response, String> {
    let body =
        serde_json::to_string(payload).map_err(|e| format!("Request serialization failed: {e}"))?;

    let request = Request::post("http://localhost:8000/auth/login")
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
        .body(body)
        .map_err(|e| format!("Failed to build request: {e}"))?;

    request
        .send()
        .await
        .map_err(|e| format!("Connection error: {e}"))
}

async fn send_register_request(
    payload: &RegisterPayload,
) -> Result<gloo_net::http::Response, String> {
    let body =
        serde_json::to_string(payload).map_err(|e| format!("Request serialization failed: {e}"))?;

    let request = Request::post("http://localhost:8000/auth/register")
        .header("Content-Type", "application/json")
        .header("Accept-Language", "en")
        .credentials(RequestCredentials::Include)
        .body(body)
        .map_err(|e| format!("Failed to build request: {e}"))?;

    request
        .send()
        .await
        .map_err(|e| format!("Connection error: {e}"))
}

async fn handle_login_response(auth: &RwSignal<AuthStore>, response: gloo_net::http::Response) {
    if !response.ok() {
        let error_message = match response.json::<ErrorResponse>().await {
            Ok(err) => err.message,
            Err(_) => {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                format!("Login failed: {status} - {text}")
            }
        };
        show_error(auth, error_message);
        return;
    }

    match response.json::<LoginResponse>().await {
        Ok(data) => {
            show_success(auth, data.message, data.data);
        }
        Err(e) => {
            let _ = response.text().await;
            show_error(auth, format!("Failed to parse response: {e}"));
        }
    }
}

async fn handle_register_response(auth: &RwSignal<AuthStore>, response: gloo_net::http::Response) {
    if !response.ok() {
        let error_message = match response.json::<ErrorResponse>().await {
            Ok(err) => err.message,
            Err(_) => {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                format!("Register failed: {status} - {text}")
            }
        };
        show_error(auth, error_message);
        return;
    }

    match response.json::<RegisterResponse>().await {
        Ok(data) => {
            show_success(auth, data.message, data.data);
        }
        Err(e) => {
            let _ = response.text().await;
            show_error(auth, format!("Failed to parse response: {e}"));
        }
    }
}

pub async fn sign_in_with_email(auth: RwSignal<AuthStore>, email: String, password: String) {
    init_auth_loading(&auth);

    let payload = LoginPayload { email, password };

    match send_login_request(&payload).await {
        Ok(response) => handle_login_response(&auth, response).await,
        Err(error) => show_error(&auth, error),
    }
}

pub async fn register_with_email(
    auth: RwSignal<AuthStore>,
    name: String,
    email: String,
    password: String,
) {
    init_auth_loading(&auth);

    let payload = RegisterPayload {
        name,
        email,
        password,
    };

    match send_register_request(&payload).await {
        Ok(response) => handle_register_response(&auth, response).await,
        Err(error) => show_error(&auth, error),
    }
}
