use crate::components::layout::{
    about::About, blog::Blog, contact::Contact, faq::Faq, footer::Footer, hero::Hero,
    navbar::Navbar, pricing::Pricing,
};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Navbar />
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
