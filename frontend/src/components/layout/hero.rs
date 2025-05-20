use crate::components::ui::navigate_button::NavigateButton;
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="section section-fullscreen">
            <header class="container text-left sm:text-center">
                <h1>
                    <span class="text-gradient">"AttendX"</span>
                    ", Smart Attendance System"
                </h1>
                <p class="mx-auto sm:max-w-[50ch]">
                    "Automate and simplify attendance tracking for schools and businesses with real-time insights and alerts."
                </p>
                <div class="flex items-start gap-4 sm:justify-center sm:items-center mt-4">
                    <NavigateButton
                        to="/auth/register"
                        class="btn btn-primary"
                        label="Get Started"
                    />
                    <NavigateButton to="/#about" class="btn btn-base" label="Learn more" />
                </div>
            </header>
        </section>
    }
}
