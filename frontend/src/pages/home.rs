use crate::components::layout::{footer::Footer, hero::Hero, navbar::Navbar};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Navbar/>
        <Hero/>
        <Footer/>
    }
}
