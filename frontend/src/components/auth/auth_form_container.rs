use leptos::prelude::*;

#[component]
pub fn AuthFormContainer(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-base-200 px-4 py-8">
            <form class="card w-full max-w-sm bg-base-100 shadow-xl p-8 space-y-6">
                {children()}
            </form>
        </div>
    }
}
