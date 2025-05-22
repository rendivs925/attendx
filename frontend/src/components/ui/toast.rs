use crate::store::auth::state::{ToastType, use_auth_store};
use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn Toast() -> impl IntoView {
    let auth = use_auth_store();

    let is_visible = move || auth.get().is_toast_visible.get();
    let toast_msg = move || auth.get().toast_message.get().clone().unwrap_or_default();

    let toast_class = move || match auth.get().toast_type.get() {
        Some(ToastType::Success) => "alert alert-success",
        Some(ToastType::Error) => "alert alert-error",
        Some(ToastType::Info) | None => "alert alert-info",
    };

    let close_toast = move || {
        auth.update(|auth| {
            auth.is_toast_visible.set(false);
            auth.toast_message.set(None);
            auth.toast_type.set(None);
        });
    };

    Effect::new(move |_| {
        if is_visible() {
            set_timeout(move || close_toast(), Duration::from_secs(3));
        }
    });

    view! {
        <div class="toast toast-end z-50">
            <Show when=is_visible>
                <div class=format!(
                    "{} shadow-lg flex justify-between gap-4 relative overflow-hidden",
                    toast_class(),
                )>
                    <span>{toast_msg}</span>
                    <button
                        aria-label="Close"
                        class="btn btn-sm btn-circle btn-ghost"
                        on:click=move |_| close_toast()
                    >

                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-4 w-4 stroke-current"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            stroke-width="2"
                            aria-hidden="true"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>
                        <span class="sr-only">Close</span>
                    </button>
                    <div class="absolute bottom-0 left-0 h-1 bg-white/70 animate-toast-progress w-full"></div>
                </div>
            </Show>
        </div>
    }
}
