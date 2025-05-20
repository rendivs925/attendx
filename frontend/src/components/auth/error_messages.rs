use leptos::prelude::*;
use validator::ValidationErrors;

#[component]
pub fn ErrorMessages(
    errors: RwSignal<Option<ValidationErrors>>,
    field: &'static str,
) -> impl IntoView {
    let messages = move || {
        if let Some(errs) = errors.get() {
            if let Some(field_errors) = errs.field_errors().get(field) {
                return field_errors
                    .iter()
                    .map(|e| {
                        e.message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| "Invalid value".to_string())
                    })
                    .collect::<Vec<String>>();
            }
        }
        vec![]
    };

    view! {
        {move || {
            messages()
                .into_iter()
                .map(|msg| view! { <p class="text-error text-sm mt-1 m-0">{msg}</p> })
                .collect::<Vec<_>>()
        }}
    }
}
