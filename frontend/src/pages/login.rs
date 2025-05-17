use crate::auth::{validate_login, LoginForm};
use leptos::prelude::*;

#[component]
pub fn Login() -> impl IntoView {
    let username = create_node_ref::<html::Input>();
    let password = create_node_ref::<html::Input>();
    let error = create_rw_signal(None::<String>);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let form = LoginForm {
            username: username.get().unwrap().value(),
            password: password.get().unwrap().value(),
        };

        match validate_login(&form) {
            Ok(_) => {
                error.set(None);
                log::info!("Login valid: {:?}", form);
            }
            Err(e) => {
                error.set(Some(format!("{:?}", e)));
            }
        }
    };

    view! {
        <form class="card w-full max-w-sm shadow-lg bg-base-100 p-6 space-y-4" on:submit=on_submit>
            <h2 class="text-2xl font-bold">"Login"</h2>
            <input
                type="text"
                placeholder="Username"
                class="input input-bordered w-full"
                node_ref=username
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
            {move || error.get().map(|e| view! { <div class="text-error text-sm">{e}</div> })}
        </form>
    }
}
