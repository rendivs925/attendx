use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::types::models::user::subscription::SubscriptionPlan;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate)]
pub struct OrganizationLimits {
    pub max_users: u32,
    pub max_attendance_logs: u32,
}

impl OrganizationLimits {
    pub fn from_plan(plan: &SubscriptionPlan) -> Self {
        match plan {
            SubscriptionPlan::Free => Self {
                max_users: 0,
                max_attendance_logs: 0,
            },
            SubscriptionPlan::Pro => Self {
                max_users: 50,
                max_attendance_logs: 25_000,
            },
            SubscriptionPlan::Premium => Self {
                max_users: 1_000,
                max_attendance_logs: 1_000_000,
            },
            SubscriptionPlan::Enterprise => Self {
                max_users: 10_000,
                max_attendance_logs: 50_000_000,
            },
        }
    }
}
