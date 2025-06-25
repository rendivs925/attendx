use crate::pages::dashboard::admin::index::AdminDashboardPage;
use components::layout::main_layout::MainLayout;
use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_meta::*;
use leptos_router::{components::*, path};
use pages::auth::register::Register;
use store::auth::state::AuthStore;

mod components;
mod constants;
mod hooks;
mod pages;
mod store;

use crate::pages::auth::login::Login;
use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    QueryClient::new().provide();

    let auth = RwSignal::new(AuthStore::default());
    provide_context(auth);

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="dark" />

        <Title text="Welcome to Attendx" />

        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <ParentRoute path=path!("") view=MainLayout>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/auth/login") view=Login />
                    <Route path=path!("/auth/register") view=Register />
                    <Route path=path!("/admin") view=AdminDashboardPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
