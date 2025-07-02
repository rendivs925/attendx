use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use strum_macros::Display;
use uuid::Uuid;

const PAID_SUBSCRIPTION_DURATION_DAYS: i64 = 30;
const FREE_SUBSCRIPTION_DURATION_YEARS: i64 = 100;
const GRACE_PERIOD_DAYS: i64 = 14;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type, Display)]
#[sqlx(type_name = "subscription_plan")]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SubscriptionPlan {
    Free,
    Pro,
    Premium,
    Enterprise,
}

impl Default for SubscriptionPlan {
    fn default() -> Self {
        Self::Free
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type, Display)]
#[sqlx(type_name = "subscription_status")]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SubscriptionStatus {
    Active,
    Expired,
    Canceled,
    GracePeriod,
}

impl Default for SubscriptionStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan: SubscriptionPlan,
    pub status: SubscriptionStatus,
    pub start_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
}

impl Subscription {
    pub fn new(user_id: Uuid, plan: SubscriptionPlan) -> Self {
        let now = Utc::now();
        let expiry_date = match plan {
            SubscriptionPlan::Free => now + Duration::days(365 * FREE_SUBSCRIPTION_DURATION_YEARS),
            _ => now + Duration::days(PAID_SUBSCRIPTION_DURATION_DAYS),
        };

        Self {
            id: Uuid::new_v4(),
            user_id,
            plan,
            status: SubscriptionStatus::Active,
            start_date: now,
            expiry_date,
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == SubscriptionStatus::Active && Utc::now() < self.expiry_date
    }

    pub fn has_full_feature_access(&self) -> bool {
        self.is_active() && self.plan != SubscriptionPlan::Free
    }

    pub fn allows_write_access(&self) -> bool {
        self.has_full_feature_access()
    }

    pub fn allows_read_access(&self) -> bool {
        matches!(
            self.status,
            SubscriptionStatus::Active | SubscriptionStatus::GracePeriod
        )
    }

    pub fn renew(&mut self, duration: Duration) {
        self.expiry_date = self.expiry_date + duration;
        self.status = SubscriptionStatus::Active;
    }

    pub fn handle_payment_failure(&mut self) {
        self.status = SubscriptionStatus::Expired;
    }

    pub fn update_status_based_on_time(&mut self) {
        if self.plan == SubscriptionPlan::Free {
            return;
        }

        let now = Utc::now();
        let grace_end = self.expiry_date + Duration::days(GRACE_PERIOD_DAYS);

        match self.status {
            SubscriptionStatus::Active if now >= self.expiry_date => {
                self.status = SubscriptionStatus::GracePeriod;
            }
            SubscriptionStatus::GracePeriod if now >= grace_end => {
                self.status = SubscriptionStatus::Expired;
            }
            _ => {}
        }
    }
}
