use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="section section-fullscreen">
            <header class="container text-center">
                <h1>
                    <span class="text-gradient">"AttendX"</span>
                    ", Smart Attendance System"
                </h1>
                <p class="mx-auto max-w-[50ch]">
                    "Automate and simplify attendance tracking for schools and businesses with real-time insights and alerts."
                </p>
                <button class="btn btn-primary">"Get Started"</button>
            </header>
        </section>
    }
}
