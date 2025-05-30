use crate::components::auth::{
    auth_form_container::AuthFormContainer, auth_redirect_text::AuthRedirectText,
    google_auth_button::GoogleAuthButton, input_field::InputField,
};
use crate::hooks::use_login_form::{LoginFormState, use_login_form};
use leptos::IntoView;
use leptos::component;
use leptos::prelude::*;
use leptos::view;

#[component]
pub fn Login() -> impl IntoView {
    let LoginFormState {
        email,
        password,
        error,
        on_submit,
    } = use_login_form();

    view! {
        <AuthFormContainer>
            <h3 class="font-bold text-center">"Login"</h3>

            <form on:submit=move |ev| on_submit.run(ev) class="flex flex-col space-y-6">
                <InputField
                    id="email"
                    label="Email"
                    input_type="email"
                    placeholder="e.g. john.doe@example.com"
                    autocomplete="email"
                    required=true
                    node_ref=email
                    errors=error
                />
                <InputField
                    id="password"
                    label="Password"
                    input_type="password"
                    placeholder="At least 8 characters"
                    autocomplete="current-password"
                    required=true
                    node_ref=password
                    errors=error
                />
                <div class="form-control pt-2">
                    <button type="submit" class="btn btn-primary w-full text-base font-semibold">
                        Login
                    </button>
                </div>
            </form>

            <div class="divider text-sm text-muted">"or"</div>

            <GoogleAuthButton />

            <AuthRedirectText
                prompt="Don't have an account? "
                link="/auth/register"
                link_label="Register here"
            />
        </AuthFormContainer>
    }
}
