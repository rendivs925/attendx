use crate::constants::API_BASE_URL;
use chrono::{DateTime, Utc};
use gloo_net::http::Request;
use leptos::component;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use shared::types::responses::{
    attendance_response::AttendanceResponse,
    organization_member_response::OrganizationMemberResponse,
    organization_response::OrganizationResponse, user_response::UserResponse,
};

fn api_url(path: &str) -> String {
    format!("{}{path}", *API_BASE_URL)
}

#[derive(Debug, Deserialize, Clone)]
struct ApiResponseWrapper<T> {
    message: String,
    data: Option<Vec<T>>,
}

async fn fetch_list<T: 'static + Clone + DeserializeOwned + PartialEq + Send + Sync>(
    url: String,
) -> Result<Vec<T>, ServerFnError> {
    let resp = Request::get(&url).send().await?;
    let wrapper = resp.json::<ApiResponseWrapper<T>>().await?;
    wrapper
        .data
        .ok_or_else(|| ServerFnError::ServerError("missing data".into()))
}

#[derive(Clone, Debug)]
enum Loadable<T> {
    Loading,
    Loaded(T),
    Error(String),
}

#[component]
pub fn AdminDashboardPage() -> impl IntoView {
    let users = RwSignal::new(Loadable::Loading);
    let orgs = RwSignal::new(Loadable::Loading);
    let attendances = RwSignal::new(Loadable::Loading);
    let members = RwSignal::new(Loadable::Loading);

    spawn_local({
        let users = users.clone();
        async move {
            let res = fetch_list::<UserResponse>(api_url("/api/users/all")).await;
            users.set(match res {
                Ok(data) => Loadable::Loaded(data),
                Err(e) => Loadable::Error(e.to_string()),
            });
        }
    });

    spawn_local({
        let orgs = orgs.clone();
        async move {
            let res = fetch_list::<OrganizationResponse>(api_url("/api/organizations/all")).await;
            orgs.set(match res {
                Ok(data) => Loadable::Loaded(data),
                Err(e) => Loadable::Error(e.to_string()),
            });
        }
    });

    spawn_local({
        let attendances = attendances.clone();
        async move {
            let res = fetch_list::<AttendanceResponse>(api_url("/api/attendances/all")).await;
            attendances.set(match res {
                Ok(data) => Loadable::Loaded(data),
                Err(e) => Loadable::Error(e.to_string()),
            });
        }
    });

    Effect::new(move |_| {
        if let Loadable::Loaded(org_list) = orgs.get() {
            if let Some(first_org) = org_list.first() {
                let members = members.clone();
                let url = api_url(&format!("/api/organization-members/all/{}", first_org.id));
                spawn_local(async move {
                    let res = fetch_list::<OrganizationMemberResponse>(url).await;
                    members.set(match res {
                        Ok(data) => Loadable::Loaded(data),
                        Err(e) => Loadable::Error(e.to_string()),
                    });
                });
            }
        }
    });

    view! {
        <div class="p-6 space-y-6">
            <h1 class="text-3xl font-bold">"Admin Dashboard"</h1>
            <p class="text-gray-600">
                "Monitor user, organization, and attendance data in real time."
            </p>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                {{
                    let users_len = if let Loadable::Loaded(data) = users.get() {
                        data.len()
                    } else {
                        0
                    };
                    view! { <SummaryCard label="Users" value=move || users_len /> }.into_any()
                }}
                {{
                    let orgs_len = if let Loadable::Loaded(data) = orgs.get() {
                        data.len()
                    } else {
                        0
                    };
                    view! { <SummaryCard label="Organizations" value=move || orgs_len /> }
                        .into_any()
                }}
                {{
                    let att_len = if let Loadable::Loaded(data) = attendances.get() {
                        data.len()
                    } else {
                        0
                    };
                    view! { <SummaryCard label="Attendances" value=move || att_len /> }.into_any()
                }}
                {{
                    let mem_len = if let Loadable::Loaded(data) = members.get() {
                        data.len()
                    } else {
                        0
                    };
                    view! { <SummaryCard label="Members (1st Org)" value=move || mem_len /> }
                        .into_any()
                }}
            </div>

            <div class="overflow-x-auto mt-6">
                <Show
                    when=move || matches!(users.get(), Loadable::Loaded(_))
                    fallback=move || view! { <div>"Loading users..."</div> }.into_any()
                >
                    {move || {
                        if let Loadable::Loaded(data) = users.get() {
                            view! {
                                <table class="table table-zebra w-full">
                                    <thead>
                                        <tr>
                                            <th>"Name"</th>
                                            <th>"Email"</th>
                                            <th>"Role"</th>
                                            <th>"Status"</th>
                                            <th>"Plan"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {data
                                            .into_iter()
                                            .map(|u| {
                                                view! {
                                                    <tr>
                                                        <td>{u.name}</td>
                                                        <td>{u.email}</td>
                                                        <td>{format!("{:?}", u.role)}</td>
                                                        <td>{format!("{:?}", u.status)}</td>
                                                        <td>{format!("{:?}", u.subscription_plan)}</td>
                                                    </tr>
                                                }
                                            })
                                            .collect_view()}
                                    </tbody>
                                </table>
                            }
                                .into_any()
                        } else {
                            view! { <div>"Error loading users."</div> }.into_any()
                        }
                    }}
                </Show>
            </div>
        </div>
    }
}

#[component]
fn SummaryCard(label: &'static str, value: impl Fn() -> usize + Copy + 'static) -> impl IntoView {
    view! {
        <div class="card bg-base-100 shadow p-4 text-center">
            <div class="text-gray-500 text-sm">{label}</div>
            <div class="text-2xl font-bold">{value()}</div>
        </div>
    }
}
