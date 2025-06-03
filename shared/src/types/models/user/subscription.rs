use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

const PAID_SUBSCRIPTION_DURATION_DAYS: i64 = 30;

const FREE_SUBSCRIPTION_DURATION_YEARS: i64 = 100;
const GRACE_PERIOD_DAYS: i64 = 14;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Subscription {
    pub plan: SubscriptionPlan,
    pub status: SubscriptionStatus,
    pub start_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
}

impl Subscription {
    pub fn new(plan: SubscriptionPlan) -> Self {
        let now = Utc::now();

        let calculated_expiry_date = match plan {
            SubscriptionPlan::Free => now + Duration::days(365 * FREE_SUBSCRIPTION_DURATION_YEARS),

            SubscriptionPlan::Pro | SubscriptionPlan::Premium | SubscriptionPlan::Enterprise => {
                now + Duration::days(PAID_SUBSCRIPTION_DURATION_DAYS)
            }
        };

        Self {
            plan,
            status: SubscriptionStatus::Active,
            start_date: now,
            expiry_date: calculated_expiry_date,
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
        match self.status {
            SubscriptionStatus::Active => true,
            SubscriptionStatus::GracePeriod => true,
            SubscriptionStatus::Expired | SubscriptionStatus::Canceled => false,
        }
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
        let grace_period_end = self.expiry_date + Duration::days(GRACE_PERIOD_DAYS);

        match self.status {
            SubscriptionStatus::Active => {
                if now >= self.expiry_date {
                    self.status = SubscriptionStatus::GracePeriod;
                }
            }
            SubscriptionStatus::GracePeriod => {
                if now >= grace_period_end {
                    self.status = SubscriptionStatus::Expired;
                }
            }

            SubscriptionStatus::Expired | SubscriptionStatus::Canceled => {}
        }
    }
}

impl Default for Subscription {
    fn default() -> Self {
        Subscription::new(SubscriptionPlan::Free)
    }
}

#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
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
