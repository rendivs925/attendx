use leptos::prelude::*;

#[component]
fn NavItem(label: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <li class="group relative">
            <a href={href}>{label}</a>
            <span class="absolute -bottom-1 left-0 w-0 transition-all h-0.5 bg-primary group-hover:w-full"></span>
        </li>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let nav_items = vec![
        ("Home", "#"),
        ("Features", "#"),
        ("Pricing", "#"),
        ("Contact", "#"),
    ];

    view! {
        <nav class="section-sm bg-base-100">
            <div class="section-container flex justify-between items-baseline">
                <h3 class="text-gradient mb-0">"AttendX"</h3>
                <ul class="flex space-x-4">
                    {nav_items.into_iter().map(|(label, href)| view! {
                        <NavItem label=label href=href />
                    }).collect_view()}
                </ul>
            </div>
        </nav>
    }
}
