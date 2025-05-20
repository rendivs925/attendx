use leptos::prelude::*;

#[component]
pub fn AuthRedirectText(
    prompt: &'static str,
    link: &'static str,
    link_label: &'static str,
) -> impl IntoView {
    view! {
        <div class="text-center text-sm pt-4">
            <span>{prompt}</span>
            <a href=link class="text-primary hover:underline font-medium">
                {link_label}
            </a>
        </div>
    }
}
