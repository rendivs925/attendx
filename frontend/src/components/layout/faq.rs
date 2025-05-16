use leptos::prelude::*;

#[component]
pub fn Faq() -> impl IntoView {
    view! {
        <section id="faq" class="py-24 bg-base-200">
            <div class="container">
                <h2 class="text-3xl font-bold mb-6">Frequently Asked Questions</h2>
                <div class="space-y-4">
                    <div class="collapse collapse-arrow bg-base-100">
                        <input type="checkbox" />
                        <div class="collapse-title text-lg font-medium">
                            How does AttendX track attendance?
                        </div>
                        <div class="collapse-content">
                            <p>
                                We use QR codes, phone numbers, or face detection to mark attendance in real-time.
                            </p>
                        </div>
                    </div>
                    <div class="collapse collapse-arrow bg-base-100">
                        <input type="checkbox" />
                        <div class="collapse-title text-lg font-medium">
                            Is there a free version?
                        </div>
                        <div class="collapse-content">
                            <p>Yes, our Starter plan is completely free for a single location.</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
