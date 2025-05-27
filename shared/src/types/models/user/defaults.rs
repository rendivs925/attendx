use crate::types::models::user::{
    role::Role, subscription::SubscriptionPlan, user_status::UserStatus,
};

use super::global_role::GlobalRole;

pub fn default_role() -> Role {
    Role::User
}

pub fn default_subscription_plan() -> SubscriptionPlan {
    SubscriptionPlan::Free
}

pub fn default_status() -> UserStatus {
    UserStatus::Active
}

pub fn default_global_role() -> GlobalRole {
    GlobalRole::User
}
