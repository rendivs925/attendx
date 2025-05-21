use leptos::prelude::*;

#[component]
pub fn GoogleAuthButton() -> impl IntoView {
    let on_click = Callback::new(|_| {
        log::info!("Google button clicked");
    });

    view! {
        <button
            type="button"
            class="btn btn-outline w-full text-base font-semibold flex items-center justify-center gap-1"
            aria-label="Login with Google"
            on:click=move |_| on_click.run(())
            style="height: 40px; padding: 0 12px; line-height: 0;"
        >
            <img src="/images/google/google.svg" alt="Google Logo" class="w-8 h-8" />
            "Continue with Google"
        </button>
    }
}
