use crate::components::auth::error_messages::ErrorMessages;
use leptos::html::Input;
use leptos::prelude::*;
use validator::ValidationErrors;

#[component]
pub fn InputField(
    id: &'static str,
    label: &'static str,
    input_type: &'static str,
    placeholder: &'static str,
    node_ref: NodeRef<Input>,
    errors: RwSignal<Option<ValidationErrors>>,
) -> impl IntoView {
    view! {
        <div class="form-control w-full space-y-2">
            <label class="label" for=id>
                <span class="label-text text-base font-medium">{label}</span>
            </label>
            <input
                id=id
                type=input_type
                placeholder=placeholder
                class="input input-bordered w-full"
                node_ref=node_ref
            />
            <ErrorMessages errors=errors field=id />
        </div>
    }
}
