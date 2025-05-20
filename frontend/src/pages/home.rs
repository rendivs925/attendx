use crate::components::layout::{
    about::About, blog::Blog, contact::Contact, faq::Faq, footer::Footer, hero::Hero,
    pricing::Pricing,
};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main>
            <Hero />
            <About />
            <Pricing />
            <Faq />
            <Blog />
            <Contact />
            <Footer />
        </main>
    }
}
