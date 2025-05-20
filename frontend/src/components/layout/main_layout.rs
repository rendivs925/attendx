use leptos::prelude::*;
use leptos_router::nested_router::Outlet;

use crate::components::layout::navbar::Navbar;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <>
            <Navbar />
            <main>
                <Outlet />
            </main>
        </>
    }
}
