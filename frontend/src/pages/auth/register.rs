use crate::components::auth::{
    auth_form_container::AuthFormContainer, auth_redirect_text::AuthRedirectText,
    google_auth_button::GoogleAuthButton, input_field::InputField,
};
use crate::hooks::use_register_form::{RegisterFormState, use_register_form};
use leptos::prelude::*;

#[component]
pub fn Register() -> impl IntoView {
    let RegisterFormState {
        name,
        email,
        password,
        password_confirmation,
        error,
        on_submit,
    } = use_register_form();

    view! {
        <AuthFormContainer>
            <h3 class="font-bold text-center">"Register"</h3>

            <form on:submit=move |ev| on_submit.run(ev) class="flex flex-col space-y-6">
                <InputField
                    id="name"
                    label="Name"
                    input_type="text"
                    placeholder="e.g. John Doe"
                    autocomplete="name"
                    required=true
                    node_ref=name
                    errors=error
                />
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
                    autocomplete="new-password"
                    required=true
                    node_ref=password
                    errors=error
                />
                <InputField
                    id="password_confirmation"
                    label="Confirm Password"
                    input_type="password"
                    placeholder="Retype your password"
                    autocomplete="new-password"
                    required=true
                    node_ref=password_confirmation
                    errors=error
                />
                <div class="form-control pt-2">
                    <button type="submit" class="btn btn-primary w-full text-base font-semibold">
                        Register
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
