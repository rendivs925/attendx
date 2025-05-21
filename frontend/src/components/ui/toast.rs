use crate::store::auth::state::use_auth_store;
use leptos::prelude::Set;
use leptos::prelude::*;
use log::info;

#[component]
pub fn Toast() -> impl IntoView {
    let auth = use_auth_store();

    let is_visible = move || auth.get().is_toast_visible.get();

    let toast_msg = move || auth.get().toast_message.get().clone().unwrap_or_default();

    create_effect(move |_| {
        info!("Is visible: {}", is_visible());
    });

    view! {
        <div class="toast toast-end">
            <Show when=is_visible>
                <div class="alert alert-info shadow-lg">
                    <div>
                        <span>{toast_msg}</span>
                    </div>
                </div>
            </Show>
        </div>
    }
}
