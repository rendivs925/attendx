use leptos::{context::use_context, prelude::RwSignal};
use serde::{Deserialize, Serialize};
use shared::types::responses::user_response::UserResponse;

#[derive(Debug, Default, Clone)]
pub struct AuthStore {
    pub is_loading: RwSignal<bool>,
    pub error_message: RwSignal<Option<String>>,
    pub toast_message: RwSignal<Option<String>>,
    pub is_toast_visible: RwSignal<bool>,
    pub toast_type: RwSignal<Option<ToastType>>,
    pub user: RwSignal<Option<UserResponse>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ToastType {
    Error,
    Success,
    Info,
}

pub fn use_auth_store() -> RwSignal<AuthStore> {
    use_context::<RwSignal<AuthStore>>().expect("AuthStore not found in context")
}
