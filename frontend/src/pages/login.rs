use leptos::html;
use leptos::prelude::*;
use leptos::web_sys;
use shared::types::requests::auth::login_request::LoginRequest;
use shared::utils::locale_utils::{Lang, Messages};
use shared::utils::validation_utils::validate_login;
use validator::ValidationErrors;

#[component]
pub fn Login() -> impl IntoView {
    let email = NodeRef::<html::Input>::new();
    let password = NodeRef::<html::Input>::new();
    let error = RwSignal::new(None::<ValidationErrors>);

    let lang = Lang::En;
    let messages = Messages::new(lang);

    fn validation_errors_to_string(errors: &ValidationErrors) -> String {
        errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |e| {
                    let message = e
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "Invalid value".to_string());
                    format!("{}: {}", field, message)
                })
            })
            .collect::<Vec<_>>()
            .join(", ")
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
        <div class="min-h-screen flex justify-center items-center bg-base-200">
            <form
                class="card w-full max-w-sm shadow-lg bg-base-100 p-6 space-y-4"
                on:submit=on_submit
            >
                <h2 class="text-2xl font-bold">"Login"</h2>
                <input
                    type="email"
                    placeholder="email"
                    class="input input-bordered w-full"
                    node_ref=email
                />
                <input
                    type="password"
                    placeholder="Password"
                    class="input input-bordered w-full"
                    node_ref=password
                />
                <button class="btn btn-primary w-full" type="submit">
                    "Login"
                </button>
                {move || {
                    error
                        .get()
                        .map(|errs| {
                            let msg = validation_errors_to_string(&errs);
                            view! { <div class="text-error text-sm">{msg}</div> }
                        })
                }}
            </form>
        </div>
    }
}
