use leptos::prelude::*;

#[component]
fn NavItem(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <li class="group relative">
            <a href=href class="normal-case text-base-content/80 group-hover:text-primary">
                {label}
            </a>
            <span class="absolute -bottom-1 left-0 w-0 transition-all h-0.5 bg-primary group-hover:w-full"></span>
        </li>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let items = vec![
        ("/", "Home"),
        ("/about", "About"),
        ("/pricing", "Pricing"),
        ("/faq", "FAQ"),
        ("/blog", "Blog"),
        ("/contact", "Contact"),
        ("/terms", "Terms"),
        ("/privacy", "Privacy"),
    ];

    view! {
        <nav class="navbar fixed z-50 w-full bg-base-100/80 backdrop-blur shadow-sm transition-all duration-300">
            <section class="container mx-auto px-4">
                <div class="flex items-center justify-between">
                    <div class="navbar-start">
                        <h5 class="text-gradient m-0 normal-case">AttendX</h5>
                    </div>
                    <div class="navbar-center hidden lg:flex">
                        <ul class="flex gap-8 px-1">
                            {items
                                .iter()
                                .map(|(href, label)| {
                                    view! { <NavItem href=*href label=*label /> }
                                })
                                .collect::<Vec<_>>()}
                        </ul>
                    </div>
                    <div class="navbar-end gap-4">
                        <button class="btn btn-base">Login</button>
                        <button class="btn btn-primary">Sign Up</button>
                    </div>
                </div>
            </section>
        </nav>
    }
}
