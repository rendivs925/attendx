use leptos::prelude::*;

use crate::components::{
    layout::{navbar::Navbar, sidebar::Sidebar},
    ui::toast::Toast,
};
use leptos_router::hooks::use_location;
use leptos_router::nested_router::Outlet;

#[component]
pub fn MainLayout() -> impl IntoView {
    let location = use_location();
    let (is_dashboard, set_is_dashboard) = signal(false);

    Effect::new(move |_| {
        let path = location.pathname.get();
        let matches = path.starts_with("/admin")
            || path.starts_with("/teacher")
            || path.starts_with("/student");
        set_is_dashboard.set(matches);
    });

    view! {
        <>
            <Show
                when=move || is_dashboard.get()
                fallback=|| {
                    view! {
                        <>
                            <Navbar />
                            <main>
                                <Outlet />
                            </main>
                        </>
                    }
                }
            >
                <Sidebar />
            </Show>

            <Toast />
        </>
    }
}
