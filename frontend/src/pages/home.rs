use crate::components::{
    attendance::attendance_ws_test::AttendanceWsTest,
    layout::{
        about::About, blog::Blog, contact::Contact, faq::Faq, footer::Footer, hero::Hero,
        pricing::Pricing,
    },
};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main>
            <Hero />
            <About />
            <Pricing />
            <AttendanceWsTest />
            <Faq />
            <Blog />
            <Contact />
            <Footer />
        </main>
    }
}
