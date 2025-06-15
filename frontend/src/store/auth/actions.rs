use crate::store::auth::api::{
    LoginPayload, RegisterPayload, send_login_request, send_register_request,
};
use crate::store::auth::handlers::{handle_login_response, handle_register_response};
use crate::store::auth::helpers::{init_auth_loading, show_error};
use crate::store::auth::state::AuthStore;
use leptos::prelude::RwSignal;

pub async fn sign_in_with_email(auth: RwSignal<AuthStore>, payload: LoginPayload) {
    init_auth_loading(&auth);

    match send_login_request(&payload).await {
        Ok(response) => handle_login_response(&auth, response).await,
        Err(error) => show_error(&auth, error),
    }
}

pub async fn register_with_email(auth: RwSignal<AuthStore>, payload: RegisterPayload) {
    init_auth_loading(&auth);

    match send_register_request(&payload).await {
        Ok(response) => handle_register_response(&auth, response).await,
        Err(error) => show_error(&auth, error),
    }
}
