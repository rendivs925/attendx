use leptos::prelude::*;
use leptos_router::nested_router::Outlet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    SuperAdmin,
    OrgOwner,
    Admin,
    Teacher,
    Student,
}

impl Role {
    pub fn can_access_admin(&self) -> bool {
        matches!(self, Role::SuperAdmin | Role::OrgOwner | Role::Admin)
    }

    pub fn can_manage_users(&self) -> bool {
        matches!(self, Role::SuperAdmin | Role::OrgOwner | Role::Admin)
    }

    pub fn is_teacher(&self) -> bool {
        matches!(self, Role::Teacher)
    }

    pub fn is_student(&self) -> bool {
        matches!(self, Role::Student)
    }
}

fn get_current_role() -> Role {
    Role::Student
}

struct SidebarItem {
    label: &'static str,
    icon_path: &'static str,
    href: &'static str,
}

fn get_sidebar_items(role: &Role) -> Vec<SidebarItem> {
    match role {
        Role::SuperAdmin | Role::OrgOwner | Role::Admin => vec![
            SidebarItem {
                label: "Dashboard",
                icon_path: "M3 3h18M3 9h18M3 15h18",
                href: "/admin/dashboard",
            },
            SidebarItem {
                label: "Users",
                icon_path: "M5 13l4 4L19 7",
                href: "/admin/users",
            },
        ],
        Role::Teacher => vec![
            SidebarItem {
                label: "Classes",
                icon_path: "M4 6h16M4 10h16M4 14h16",
                href: "/teacher/classes",
            },
            SidebarItem {
                label: "Attendance",
                icon_path: "M9 12l2 2 4-4",
                href: "/teacher/attendance",
            },
        ],
        Role::Student => vec![
            SidebarItem {
                label: "My Schedule",
                icon_path: "M3 7h18M3 12h18M3 17h18",
                href: "/student/schedule",
            },
            SidebarItem {
                label: "Grades",
                icon_path: "M5 13l4 4L19 7",
                href: "/student/grades",
            },
        ],
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="drawer">
            <input id="my-drawer" type="checkbox" class="drawer-toggle" />
            <div class="drawer-content">
                <div class="p-2">
                    <label for="my-drawer" class="btn btn-ghost btn-circle drawer-button">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M4 6h16M4 12h16M4 18h16"
                            />
                        </svg>
                    </label>
                </div>
                <Outlet />
            </div>
            <div class="drawer-side">
                <label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
                <SidebarMenu />
            </div>
        </div>
    }
}

#[component]
fn SidebarMenu() -> impl IntoView {
    let role = get_current_role();
    let items = get_sidebar_items(&role);

    view! {
        <ul class="menu bg-base-200 text-base-content min-h-full w-80 p-4 gap-2">
            {items
                .into_iter()
                .map(|item| {
                    view! {
                        <li>
                            <a href=item.href>
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-5 w-5 mr-2"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d=item.icon_path
                                    />
                                </svg>
                                {item.label}
                            </a>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}
