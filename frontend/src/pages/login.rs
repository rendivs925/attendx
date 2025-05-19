use leptos::html;
use leptos::prelude::*;
use leptos::web_sys;
use shared::types::requests::auth::login_request::LoginRequest;
use shared::utils::locale_utils::{Lang, Messages};
use shared::utils::validation_utils::validate_login;
use validator::ValidationErrors;

#[component]
fn ErrorList(errors: Vec<String>) -> impl IntoView {
    view! {
        <ul class="text-error text-sm mt-1 space-y-1">
            {errors.into_iter().map(|msg| view! { <li>{msg}</li> }).collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
pub fn Login() -> impl IntoView {
    let email = NodeRef::<html::Input>::new();
    let password = NodeRef::<html::Input>::new();
    let error = RwSignal::new(None::<ValidationErrors>);

    let lang = Lang::En;
    let messages = Messages::new(lang);

    fn extract_field_errors<'a>(field: &'a str, errors: &'a ValidationErrors) -> Vec<String> {
        errors
            .field_errors()
            .get(field)
            .map(|field_errors| {
                field_errors
                    .iter()
                    .map(|e| {
                        e.message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| "Invalid value".to_string())
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    let on_submit = {
        let error = error.clone();
        let messages = messages;
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
                Err(e) => {
                    error.set(Some(e));
                }
            }
        }
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-base-200 px-4 py-8">
            <form
                class="card w-full max-w-sm bg-base-100 shadow-xl p-8 space-y-6"
                on:submit=on_submit
            >
                <h4 class="font-bold text-center">"Login"</h4>

                <div class="form-control w-full space-y-2">
                    <label class="label" for="email">
                        <span class="label-text text-base font-medium">"Email"</span>
                    </label>
                    <input
                        id="email"
                        type="email"
                        placeholder="you@example.com"
                        class="input input-bordered w-full"
                        node_ref=email
                    />
                    {move || {
                        let errs = error.get();
                        let email_errors = errs
                            .as_ref()
                            .map(|e| extract_field_errors("email", e))
                            .unwrap_or_default();
                        view! { <ErrorList errors=email_errors /> }
                    }}
                </div>

                <div class="form-control w-full space-y-2">
                    <label class="label" for="password">
                        <span class="label-text text-base font-medium">"Password"</span>
                    </label>
                    <input
                        id="password"
                        type="password"
                        placeholder="••••••••"
                        class="input input-bordered w-full"
                        node_ref=password
                    />
                    {move || {
                        let errs = error.get();
                        let password_errors = errs
                            .as_ref()
                            .map(|e| extract_field_errors("password", e))
                            .unwrap_or_default();
                        view! { <ErrorList errors=password_errors /> }
                    }}
                </div>

                <div class="form-control pt-2">
                    <button type="submit" class="btn btn-primary w-full text-base font-semibold">
                        "Login"
                    </button>
                </div>

                <div class="divider text-sm text-muted">or</div>

                <div class="form-control">

                    <button
                        type="button"
                        class="btn btn-outline w-full text-base font-semibold flex items-center justify-center gap-1"
                        aria-label="Login with Google"
                        on:click=|_| {
                            log::info!("Google login clicked");
                        }
                        style="height: 40px; padding: 0 12px; line-height: 0;"
                    >
                        <img src="/images/google/google.svg" alt="Google Logo" class="w-8 h-8" />
                        "Login with Google"
                    </button>

                </div>

                <div class="text-center text-sm pt-4">
                    <span>"Don't have an account? "</span>
                    <a href="/register" class="text-primary hover:underline font-medium">
                        "Register here"
                    </a>
                </div>
            </form>
        </div>
    }
}
