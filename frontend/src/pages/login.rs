use leptos::html;
use leptos::prelude::*;
use leptos::web_sys;
use shared::types::requests::auth::login_request::LoginRequest;
use shared::utils::locale_utils::{Lang, Messages};
use shared::utils::validation_utils::validate_login;
use std::sync::Arc;
use validator::ValidationErrors;

use crate::components::auth::{
    auth_form_container::AuthFormContainer, auth_redirect_text::AuthRedirectText,
    google_auth_button::GoogleAuthButton, input_field::InputField,
};

#[component]
pub fn Login() -> impl IntoView {
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
        }
    };

    let on_google_click = Callback::new(|_| {
        log::info!("Google login clicked");
    });

    view! {
        <AuthFormContainer>
            <h3 class="font-bold text-center">"Login"</h3>

            <form on:submit=on_submit.clone() class="flex flex-col space-y-6">
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
                        "Login"
                    </button>
                </div>
            </form>

            <div class="divider text-sm text-muted">"or"</div>

            <GoogleAuthButton on_click=on_google_click />

            <AuthRedirectText
                prompt="Don't have an account? "
                link="/auth/register"
                link_label="Register here"
            />
        </AuthFormContainer>
    }
}
