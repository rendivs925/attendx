use leptos::prelude::*;

#[component]
pub fn Privacy() -> impl IntoView {
    view! {
        <section id="privacy" class="py-24 bg-base-200">
            <div class="container mx-auto px-4 max-w-3xl">
                <h2 class="text-3xl font-bold mb-6">Privacy Policy</h2>
                <p class="text-base-content/80">
                    We take your privacy seriously. AttendX only stores essential data, securely encrypted, and never shared without consent.
                </p>
            </div>
        </section>
    }
}
