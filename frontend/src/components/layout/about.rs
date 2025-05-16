use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="py-24 bg-base-200">
            <div class="container">
                <h2 class="text-3xl font-bold mb-6">Why AttendX?</h2>
                <p class="text-lg text-base-content/80">
                    AttendX helps schools and businesses track attendance in real-time,
                    reduce manual errors, and notify stakeholders instantly.
                </p>
                <ul class="mt-6 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                    <li class="card p-4 shadow-md bg-base-100">"⏱️ Real-time Attendance"</li>
                    <li class="card p-4 shadow-md bg-base-100">"📊 Detailed Reports"</li>
                    <li class="card p-4 shadow-md bg-base-100">"📞 Parent Notifications"</li>
                </ul>
            </div>
        </section>
    }
}
