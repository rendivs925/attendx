use crate::types::models::user::defaults::default_subscription_plan;
use crate::types::models::user::subscription::SubscriptionPlan;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
    #[serde(default = "default_subscription_plan")]
    pub subscription_plan: SubscriptionPlan,
}
