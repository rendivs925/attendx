use leptos::prelude::*;

#[component]
pub fn SectionContainer(children: Children) -> impl IntoView {
    view! {
        <div class="w-full max-w-7xl mx-auto px-4">
            {children()}
        </div>
    }
}
