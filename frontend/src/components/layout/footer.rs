use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="section-sm bg-base-200">
            <div class="section-container">
                <p class="mb-0">
                    "© 2025 AttendX. All rights reserved."
                </p>
            </div>
        </footer>
    }
}
