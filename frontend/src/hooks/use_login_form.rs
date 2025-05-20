use leptos::html;
use leptos::prelude::*;
use leptos::web_sys::SubmitEvent;
use shared::types::requests::auth::login_request::LoginRequest;
use shared::utils::locale_utils::{Lang, Messages};
use shared::utils::validation_utils::validate_login;
use std::sync::Arc;
use validator::ValidationErrors;

pub struct LoginFormState {
    pub email: NodeRef<html::Input>,
    pub password: NodeRef<html::Input>,
    pub error: RwSignal<Option<ValidationErrors>>,
    pub on_submit: Callback<SubmitEvent>,
}

pub fn use_login_form() -> LoginFormState {
    let email = NodeRef::<html::Input>::new();
    let password = NodeRef::<html::Input>::new();
    let error = RwSignal::new(None::<ValidationErrors>);
    let messages = Arc::new(Messages::new(Lang::En)); // Rc is enough here

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let messages = messages.clone();

        Callback::new(move |ev: SubmitEvent| {
            ev.prevent_default();

            let email_value = email.get().map(|el| el.value()).unwrap_or_default();
            let password_value = password.get().map(|el| el.value()).unwrap_or_default();

            match validate_login(&email_value, &password_value, &messages) {
                Ok(_) => {
                    error.set(None);
                    let form_data = LoginRequest {
                        email: email_value,
                        password: password_value,
                    };
                    log::info!("Login valid: {:?}", form_data);
                }
                Err(e) => error.set(Some(e)),
            }
        })
    };

    LoginFormState {
        email,
        password,
        error,
        on_submit,
    }
}
