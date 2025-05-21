use leptos::html;
use leptos::prelude::*;
use leptos::web_sys;
use shared::types::requests::auth::register_request::RegisterRequest;
use shared::utils::locale_utils::{Lang, Messages};
use shared::utils::validation_utils::validate_register;
use std::sync::Arc;
use validator::ValidationErrors;

use crate::components::auth::{
    auth_form_container::AuthFormContainer, auth_redirect_text::AuthRedirectText,
    google_auth_button::GoogleAuthButton, input_field::InputField,
};

#[component]
pub fn Register() -> impl IntoView {
    let name = NodeRef::<html::Input>::new();
    let email = NodeRef::<html::Input>::new();
    let password = NodeRef::<html::Input>::new();
    let error = RwSignal::new(None::<ValidationErrors>);

    let lang = Lang::En;
    let messages = Arc::new(Messages::new(lang));

    let on_submit = {
        let error = error.clone();
        let messages = messages.clone();

        move |ev: web_sys::SubmitEvent| {
            ev.prevent_default();
            let name_value = name.get().map(|el| el.value()).unwrap_or_default();
            let email_value = email.get().map(|el| el.value()).unwrap_or_default();
            let password_value = password.get().map(|el| el.value()).unwrap_or_default();

            match validate_register(&name_value, &email_value, &password_value, &messages) {
                Ok(_) => {
                    error.set(None);
                    let form_data = RegisterRequest {
                        name: name_value,
                        email: email_value,
                        password: password_value,
                        ..Default::default()
                    };
                    log::info!("Register valid: {:?}", form_data);
                }
                Err(e) => error.set(Some(e)),
            }
        }
    };

    view! {
        <AuthFormContainer>
            <h3 class="font-bold text-center">"Register"</h3>

            <form on:submit=on_submit.clone() class="flex flex-col space-y-6">
                <InputField
                    id="name"
                    label="Name"
                    input_type="text"
                    placeholder="Your full name"
                    node_ref=name
                    errors=error
                />
                <InputField
                    id="email"
                    label="Email"
                    input_type="email"
                    placeholder="you@example.com"
                    node_ref=email
                    errors=error
                />
                <InputField
                    id="password"
                    label="Password"
                    input_type="password"
                    placeholder="••••••••"
                    node_ref=password
                    errors=error
                />
                <div class="form-control pt-2">
                    <button type="submit" class="btn btn-primary w-full text-base font-semibold">
                        "Register"
                    </button>
                </div>
            </form>

            <div class="divider text-sm text-muted">"or"</div>

            <GoogleAuthButton />

            <AuthRedirectText
                prompt="Already have an account? "
                link="/auth/login"
                link_label="Login here"
            />
        </AuthFormContainer>
    }
}
