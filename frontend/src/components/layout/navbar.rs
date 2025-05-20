use crate::components::ui::navigate_button::NavigateButton;
use leptos::prelude::*;

#[component]
fn NavItem(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <li class="group relative inline-block w-auto w-min">
            <a href=href class="normal-case text-base-content/80 group-hover:text-primary">
                {label}
            </a>
            <span class="block h-0.5 bg-primary transition-all duration-300 w-0 group-hover:w-full"></span>
        </li>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let items = vec![
        ("/", "Home"),
        ("/#about", "About"),
        ("/#pricing", "Pricing"),
        ("/#faq", "FAQ"),
        ("/#blog", "Blog"),
        ("/#contact", "Contact"),
    ];

    let is_open = RwSignal::new(false);

    view! {
        <nav class="navbar fixed z-50 w-full bg-base-100/80 backdrop-blur shadow-sm transition-all duration-300">
            <section class="container mx-auto px-4">
                <div class="flex items-center justify-between py-2">
                    <div class="navbar-start">
                        <a href="/" class="no-underline">
                            <h5 class="text-gradient m-0 normal-case">AttendX</h5>
                        </a>
                    </div>

                    <div class="navbar-center hidden lg:flex">
                        <ul class="flex gap-8 px-1">
                            {items
                                .iter()
                                .map(|(href, label)| view! { <NavItem href=*href label=*label /> })
                                .collect::<Vec<_>>()}
                        </ul>
                    </div>

                    <div class="navbar-end flex items-center gap-4">
                        <div class="hidden lg:flex gap-4">
                            <NavigateButton to="/auth/login" class="btn btn-base" label="Login" />
                            <NavigateButton
                                to="/auth/register"
                                class="btn btn-primary"
                                label="Register"
                            />
                        </div>

                        <button
                            class="lg:hidden cursor-pointer focus:outline-none"
                            on:click=move |_| is_open.update(|v| *v = !*v)
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-6 w-6 text-base-content"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 6h16M4 12h16M4 18h16"
                                />
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="lg:hidden mt-2 pb-4" class:hidden=move || !is_open.get()>
                    <ul class="flex flex-col gap-4">
                        {items
                            .iter()
                            .map(|(href, label)| view! { <NavItem href=*href label=*label /> })
                            .collect::<Vec<_>>()} <div class="flex flex-col gap-2 mt-4">
                            <NavigateButton
                                to="/auth/login"
                                class="btn btn-base w-full"
                                label="Login"
                            />
                            <NavigateButton
                                to="/auth/register"
                                class="btn btn-primary w-full"
                                label="Register"
                            />
                        </div>
                    </ul>
                </div>
            </section>
        </nav>
    }
}
