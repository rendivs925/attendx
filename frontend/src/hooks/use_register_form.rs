use crate::store::auth::actions::register_with_email;
use crate::store::auth::api::RegisterPayload;
use crate::store::auth::state::use_auth_store;
use leptos::callback::Callback;
use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::{HtmlInputElement, SubmitEvent};
use leptos_router::hooks::use_navigate;
use shared::types::requests::auth::validation_request::ValidationRequest;
use shared::utils::locale_utils::{Lang, MessageLookup, MessagesHttp};
use shared::utils::validation_utils::validate_data;
use std::sync::Arc;
use validator::{ValidationError, ValidationErrors};

pub struct RegisterFormState {
    pub name: NodeRef<html::Input>,
    pub email: NodeRef<html::Input>,
    pub password: NodeRef<html::Input>,
    pub password_confirmation: NodeRef<html::Input>,
    pub error: RwSignal<Option<ValidationErrors>>,
    pub on_submit: Callback<SubmitEvent>,
}

pub fn use_register_form() -> RegisterFormState {
    let auth_store = use_auth_store();
    let name = NodeRef::new();
    let email = NodeRef::new();
    let password = NodeRef::new();
    let password_confirmation = NodeRef::new();
    let error = RwSignal::new(None);
    let messages: RwSignal<Option<Arc<dyn MessageLookup>>> = RwSignal::new(None);
    let navigate = use_navigate();

    spawn_local({
        let messages = messages.clone();
        async move {
            let loaded_messages = MessagesHttp::new(Lang::En).await;
            messages.set(Some(Arc::new(loaded_messages) as Arc<dyn MessageLookup>));
        }
    });

    let on_submit = Callback::new({
        let name = name.clone();
        let email = email.clone();
        let password = password.clone();
        let password_confirmation = password_confirmation.clone();
        let error = error.clone();
        let messages = messages.clone();
        let auth_store = auth_store.clone();

        move |ev: SubmitEvent| {
            ev.prevent_default();

            let Some(msgs_arc) = messages.get() else {
                leptos::logging::warn!("Messages not loaded yet for registration form validation.");
                return;
            };

            let lookup: &dyn MessageLookup = &*msgs_arc;

            let name_value = name
                .get()
                .map(|el: HtmlInputElement| el.value())
                .unwrap_or_default();
            let email_value = email
                .get()
                .map(|el: HtmlInputElement| el.value())
                .unwrap_or_default();
            let password_value = password
                .get()
                .map(|el: HtmlInputElement| el.value())
                .unwrap_or_default();
            let password_confirmation_value = password_confirmation
                .get()
                .map(|el: HtmlInputElement| el.value())
                .unwrap_or_default();

            let req = ValidationRequest {
                name: Some(name_value.clone()),
                email: Some(email_value.clone()),
                password: Some(password_value.clone()),
                password_confirmation: Some(password_confirmation_value.clone()),
            };

            match validate_data(&req, lookup) {
                Ok(_) => {
                    if password_value != password_confirmation_value {
                        let mut errors = ValidationErrors::new();
                        errors.add(
                            "password_confirmation",
                            ValidationError::new("password_mismatch"),
                        );
                        error.set(Some(errors));
                        return;
                    }

                    let navigate = navigate.clone();

                    error.set(None);

                    spawn_local(async move {
                        let register_payload = RegisterPayload {
                            name: name_value,
                            email: email_value,
                            password: password_value,
                            password_confirmation: password_confirmation_value,
                        };

                        let res = register_with_email(auth_store, register_payload).await;

                        if res.is_ok() {
                            navigate("/auth/login".into(), Default::default());
                        }
                    });
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
        }
    });

    RegisterFormState {
        name,
        email,
        password,
        password_confirmation,
        error,
        on_submit,
    }
}
