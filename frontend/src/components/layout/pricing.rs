use leptos::prelude::*;

struct Plan<'a> {
    name: &'a str,
    price: &'a str,
    features: &'a [&'a str],
    button_text: &'a str,
}

#[component]
pub fn Pricing() -> impl IntoView {
    let plans = [
        Plan {
            name: "Starter",
            price: "$0 / month",
            features: &[
                "✔️ 1 Location",
                "✔️ Basic Reports",
                "❌ Exportable Reports",
                "❌ Priority Support",
                "❌ Advanced Analytics",
            ],
            button_text: "Get Started",
        },
        Plan {
            name: "Pro",
            price: "$19 / month",
            features: &[
                "✔️ Multiple Locations",
                "✔️ Exportable Reports",
                "✔️ Priority Support",
                "❌ Dedicated Support",
                "❌ SLA Guarantee",
            ],
            button_text: "Choose Pro",
        },
        Plan {
            name: "Enterprise",
            price: "Custom",
            features: &[
                "✔️ Unlimited Locations",
                "✔️ Advanced Analytics",
                "✔️ Dedicated Support",
                "✔️ SLA Guarantee",
                "✔️ Priority Support",
            ],
            button_text: "Contact Sales",
        },
    ];

    view! {
        <section id="pricing" class="py-24 bg-base-100">
            <div class="container">
                <h2 class="text-3xl font-bold mb-6">Pricing</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    {plans
                        .iter()
                        .map(|plan| {
                            view! {
                                <div class="card p-6 shadow-lg bg-base-200">
                                    <h3 class="text-xl font-semibold mb-4">{plan.name}</h3>
                                    <p class="mb-4">{plan.price}</p>
                                    <ul class="mb-4 space-y-1">
                                        {plan
                                            .features
                                            .iter()
                                            .map(|&feature| view! { <li>{feature}</li> })
                                            .collect::<Vec<_>>()}
                                    </ul>
                                    <button class="btn btn-primary w-full">
                                        {plan.button_text}
                                    </button>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
