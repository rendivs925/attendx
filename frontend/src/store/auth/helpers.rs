use crate::store::auth::state::{AuthStore, ToastType};
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Update;
use shared::types::responses::user_response::UserResponse;

pub fn init_auth_loading(auth: &RwSignal<AuthStore>) {
    auth.update(|auth| {
        auth.is_loading.set(true);
        auth.error_message.set(None);
        auth.toast_message.set(None);
        auth.is_toast_visible.set(false);
        auth.toast_type.set(None);
    });
}

pub fn show_error(auth: &RwSignal<AuthStore>, message: String) {
    auth.update(|auth| {
        auth.error_message.set(Some(message.clone()));
        auth.toast_message.set(Some(message));
        auth.toast_type.set(Some(ToastType::Error));
        auth.is_toast_visible.set(true);
        auth.is_loading.set(false);
        auth.user.set(None);
    });
}

pub fn show_success(auth: &RwSignal<AuthStore>, message: String, user: UserResponse) {
    auth.update(|auth| {
        auth.user.set(Some(user));
        auth.error_message.set(None);
        auth.toast_message.set(Some(message));
        auth.toast_type.set(Some(ToastType::Success));
        auth.is_toast_visible.set(true);
        auth.is_loading.set(false);
    });
}
