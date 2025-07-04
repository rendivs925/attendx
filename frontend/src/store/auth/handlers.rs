use crate::store::auth::helpers::{show_error, show_success};
use crate::store::auth::state::AuthStore;
use leptos::prelude::RwSignal;
use shared::types::responses::user_response::UserResponse;

pub async fn handle_login_response(
    auth: &RwSignal<AuthStore>,
    user: UserResponse,
    message: String,
) {
    show_success(auth, message, user);
}

pub async fn handle_register_response(
    auth: &RwSignal<AuthStore>,
    user: UserResponse,
    message: String,
) {
    show_success(auth, message, user);
}
