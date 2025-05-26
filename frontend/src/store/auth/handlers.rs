use crate::store::auth::api::{ErrorResponse, LoginResponse, RegisterResponse};
use crate::store::auth::helpers::{show_error, show_success};
use crate::store::auth::state::AuthStore;
use gloo_net::http::Response;
use leptos::prelude::RwSignal;
use shared::types::responses::user_response::UserResponse;

pub async fn handle_login_response(auth: &RwSignal<AuthStore>, response: Response) {
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

    match response.json::<LoginResponse<UserResponse>>().await {
        Ok(data) => {
            show_success(auth, data.message, data.data);
        }
        Err(e) => {
            let _ = response.text().await;
            show_error(auth, format!("Failed to parse response: {e}"));
        }
    }
}

pub async fn handle_register_response(auth: &RwSignal<AuthStore>, response: Response) {
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

    match response.json::<RegisterResponse<UserResponse>>().await {
        Ok(data) => {
            show_success(auth, data.message, data.data);
        }
        Err(e) => {
            let _ = response.text().await;
            show_error(auth, format!("Failed to parse response: {e}"));
        }
    }
}
