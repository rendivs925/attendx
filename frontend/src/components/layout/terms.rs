use leptos::prelude::*;

#[component]
pub fn Terms() -> impl IntoView {
    view! {
        <section id="terms" class="py-24 bg-base-100">
            <div class="container mx-auto px-4 max-w-3xl">
                <h2 class="text-3xl font-bold mb-6">Terms & Conditions</h2>
                <p class="text-base-content/80">
                    By using AttendX, you agree to comply with our usage policy, ensure data accuracy, and avoid unauthorized access.
                </p>
            </div>
        </section>
    }
}
