use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn NavigateButton(
    to: &'static str,
    label: &'static str,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let navigate = use_navigate();

    let onclick = move |_| navigate(to, Default::default());

    let class = class.unwrap_or("btn btn-primary");

    view! {
        <button type="button" class=class on:click=onclick>
            {label}
        </button>
    }
}
