use crate::store::auth::actions::sign_in_with_email;
use crate::store::auth::api::LoginPayload;
use crate::store::auth::state::use_auth_store;
use leptos::callback::Callback;
use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::{HtmlInputElement, SubmitEvent};
use shared::types::requests::auth::validation_request::ValidationRequest;
use shared::utils::locale_utils::{Lang, MessageLookup, MessagesHttp};
use shared::utils::validation_utils::validate_data;
use std::sync::Arc;
use validator::ValidationErrors;

pub struct LoginFormState {
    pub email: NodeRef<html::Input>,
    pub password: NodeRef<html::Input>,
    pub error: RwSignal<Option<ValidationErrors>>,
    pub on_submit: Callback<SubmitEvent>,
}

pub fn use_login_form() -> LoginFormState {
    let auth_store = use_auth_store();
    let email = NodeRef::new();
    let password = NodeRef::new();
    let error = RwSignal::new(None);
    let messages: RwSignal<Option<Arc<dyn MessageLookup>>> = RwSignal::new(None);

    spawn_local({
        let messages = messages.clone();
        async move {
            let loaded_messages = MessagesHttp::new(Lang::En).await;

            messages.set(Some(Arc::new(loaded_messages) as Arc<dyn MessageLookup>));
        }
    });

    let on_submit = Callback::new({
        let messages = messages.clone();
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let auth_store = auth_store.clone();

        move |ev: SubmitEvent| {
            ev.prevent_default();

            let Some(msgs_arc) = messages.get() else {
                leptos::logging::warn!("Messages not loaded yet for login form validation.");
                return;
            };

            let lookup: &dyn MessageLookup = &*msgs_arc;

            let email_value = email
                .get()
                .map(|el: HtmlInputElement| el.value())
                .unwrap_or_default();

            let password_value = password
                .get()
                .map(|el: HtmlInputElement| el.value())
                .unwrap_or_default();

            let req = ValidationRequest {
                email: Some(email_value.clone()),
                password: Some(password_value.clone()),
                ..Default::default()
            };

            match validate_data(&req, lookup) {
                Ok(_) => {
                    error.set(None);

                    spawn_local(async move {
                        let login_payload = LoginPayload {
                            email: email_value,
                            password: password_value,
                        };
                        sign_in_with_email(auth_store, login_payload).await;
                    });
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
        }
    });

    LoginFormState {
        email,
        password,
        error,
        on_submit,
    }
}
