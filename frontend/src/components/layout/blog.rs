use leptos::prelude::*;

#[component]
pub fn Blog() -> impl IntoView {
    let blog_posts = [
        BlogPost {
            title: "How Smart Attendance Saves Time",
            content: "Discover how automation transforms school and office routines.",
        },
        BlogPost {
            title: "The Future of Attendance",
            content: "Explore AI and biometrics in modern attendance systems.",
        },
        BlogPost {
            title: "Improving Accuracy with Smart Systems ",
            content: "Learn how smart attendance systems minimize errors.",
        },
    ];

    view! {
        <section id="blog" class="py-24 bg-base-100">
            <div class="container">
                <h2 class="text-3xl font-bold mb-6">Latest Articles</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    {blog_posts
                        .iter()
                        .map(|post| {
                            view! {
                                <div class="card bg-base-200 p-4 shadow-md">
                                    <h3 class="text-xl font-semibold mb-2">{post.title}</h3>
                                    <p class="text-base-content/80">{post.content}</p>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

struct BlogPost {
    title: &'static str,
    content: &'static str,
}
