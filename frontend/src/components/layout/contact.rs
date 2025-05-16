use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section id="contact" class="py-24 bg-base-200">
            <div class="container max-w-2xl">
                <h2 class="text-3xl font-bold mb-6">Get in Touch</h2>
                <form class="space-y-4">
                    <input type="text" placeholder="Name" class="input input-bordered w-full" />
                    <input type="email" placeholder="Email" class="input input-bordered w-full" />
                    <textarea
                        placeholder="Message"
                        class="textarea textarea-bordered w-full"
                    ></textarea>
                    <button class="btn btn-primary w-full">Send Message</button>
                </form>
            </div>
        </section>
    }
}
