use crate::store::auth::state::{AuthStore, AuthUser};
use gloo_net::http::Request;
use leptos::prelude::create_rw_signal;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Update;
use leptos::web_sys::{console, RequestCredentials};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct LoginResponse {
    name: String,
    email: String,
    avatar_url: Option<String>,
    message: String,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    message: String,
}

pub async fn sign_in_with_email(auth: RwSignal<AuthStore>, email: String, password: String) {
    auth.update(|auth| {
        auth.is_loading.set(true);
        auth.error_message.set(None);
        auth.toast_message.set(None);
        auth.is_toast_visible.set(false);
    });

    let show_error = |auth: &RwSignal<AuthStore>, message: String| {
        console::log_1(&message.clone().into());
        auth.update(|auth| {
            auth.error_message.set(Some(message.clone()));
            auth.toast_message.set(Some(message.clone()));
            auth.is_toast_visible.set(true);
            auth.is_loading.set(false);
        });
    };

    let payload = LoginPayload { email, password };
    let body = match serde_json::to_string(&payload) {
        Ok(json) => json,
        Err(e) => return show_error(&auth, format!("Request serialization failed: {e}")),
    };

    let request = match Request::post("http://localhost:8000/auth/login")
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
        .body(body)
    {
        Ok(req) => req,
        Err(e) => return show_error(&auth, format!("Failed to build request: {e}")),
    };

    let response = match request.send().await {
        Ok(res) => res,
        Err(e) => return show_error(&auth, format!("Connection error: {e}")),
    };

    if !response.ok() {
        let error_message = match response.json::<ErrorResponse>().await {
            Ok(err) => err.message,
            Err(_) => {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                format!("Login failed: {status} - {text}")
            }
        };
        auth.update(|auth| {
            auth.user.set(None);
        });
        return show_error(&auth, error_message);
    }

    match response.json::<LoginResponse>().await {
        Ok(data) => {
            auth.update(|auth| {
                auth.user.set(Some(AuthUser {
                    name: create_rw_signal(data.name),
                    email: create_rw_signal(data.email),
                    avatar_url: create_rw_signal(data.avatar_url),
                }));
                auth.error_message.set(None);
                auth.toast_message.set(Some(data.message));
                auth.is_toast_visible.set(true);
            });
        }
        Err(e) => {
            auth.update(|auth| {
                auth.user.set(None);
            });
            return show_error(&auth, format!("Failed to parse response: {e}"));
        }
    }

    auth.update(|auth| {
        auth.is_loading.set(false);
    });
}
