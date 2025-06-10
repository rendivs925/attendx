use crate::store::auth::actions::register_with_email;
use crate::store::auth::state::use_auth_store;
use leptos::callback::Callback;
use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::SubmitEvent;
use shared::types::requests::auth::validation_request::ValidationRequest;
use shared::utils::locale_utils::{Lang, Messages};
use shared::utils::validation_utils::validate_data;
use std::sync::Arc;
use validator::ValidationErrors;

pub struct RegisterFormState {
    pub name: NodeRef<html::Input>,
    pub email: NodeRef<html::Input>,
    pub password: NodeRef<html::Input>,
    pub error: RwSignal<Option<ValidationErrors>>,
    pub on_submit: Callback<SubmitEvent>,
}

pub fn use_register_form() -> RegisterFormState {
    let auth = use_auth_store();
    let name = NodeRef::<html::Input>::new();
    let email = NodeRef::<html::Input>::new();
    let password = NodeRef::<html::Input>::new();
    let error = RwSignal::new(None);
    let messages = Arc::new(Messages::new(Lang::En));

    let on_submit = {
        let name = name.clone();
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let messages = messages.clone();
        let auth = auth.clone();

        Callback::new(move |ev: SubmitEvent| {
            ev.prevent_default();
            let name_value = name.get().map(|el| el.value()).unwrap_or_default();
            let email_value = email.get().map(|el| el.value()).unwrap_or_default();
            let password_value = password.get().map(|el| el.value()).unwrap_or_default();

            let req = ValidationRequest {
                name: Some(name_value.clone()),
                email: Some(email_value.clone()),
                password: Some(password_value.clone()),
                ..Default::default()
            };

            match validate_data(&req, &messages) {
                Ok(_) => {
                    error.set(None);
                    spawn_local(async move {
                        register_with_email(auth, name_value, email_value, password_value).await;
                    });
                }
                Err(e) => error.set(Some(e)),
            }
        })
    };

    RegisterFormState {
        name,
        email,
        password,
        error,
        on_submit,
    }
}
