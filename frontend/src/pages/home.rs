use crate::components::layout::{
    about::About, blog::Blog, contact::Contact, faq::Faq, footer::Footer, hero::Hero,
    navbar::Navbar, pricing::Pricing, privacy::Privacy, terms::Terms,
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
            // <Terms />
            // <Privacy />
            <Footer />
        </main>
    }
}
