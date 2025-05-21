use leptos::{
    context::{provide_context, use_context},
    prelude::RwSignal,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone)]
pub struct AuthStore {
    pub is_loading: RwSignal<bool>,
    pub error_message: RwSignal<Option<String>>,
    pub toast_message: RwSignal<Option<String>>,
    pub is_toast_visible: RwSignal<bool>,
    pub user: RwSignal<Option<AuthUser>>,
}

#[derive(Debug, Clone, Default)]
pub struct AuthUser {
    pub name: RwSignal<String>,
    pub email: RwSignal<String>,
    pub avatar_url: RwSignal<Option<String>>,
}

pub fn use_auth_store() -> RwSignal<AuthStore> {
    use_context::<RwSignal<AuthStore>>().expect("AuthStore not found in context")
}
