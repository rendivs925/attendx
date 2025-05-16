use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer footer-center p-6 bg-neutral-900 text-neutral-200">
            <div class="container flex flex-col md:flex-row justify-between items-center gap-4 max-w-4xl">
                <p class="text-sm m-0">"© 2025 AttendX. All rights reserved."</p>

                <nav class="flex space-x-6">
                    <a href="/" class="link link-hover text-neutral-200">
                        "Home"
                    </a>
                    <a href="#about" class="link link-hover text-neutral-200">
                        "About"
                    </a>
                    <a href="#contact" class="link link-hover text-neutral-200">
                        "Contact"
                    </a>
                </nav>

                <div class="flex space-x-4">
                    <a
                        href="mailto:support@attendx.com"
                        class="link link-hover text-neutral-200"
                        aria-label="Email"
                    >
                        "📧"
                    </a>
                    <a
                        href="https://twitter.com/attendx"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="link link-hover text-neutral-200"
                        aria-label="Twitter"
                    >
                        "🐦"
                    </a>
                    <a
                        href="https://linkedin.com/company/attendx"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="link link-hover text-neutral-200"
                        aria-label="LinkedIn"
                    >
                        "🔗"
                    </a>
                </div>
            </div>
        </footer>
    }
}
