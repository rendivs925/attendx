use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "backend")]
use sqlx::FromRow;

use crate::types::models::attendance::{
    attendance_method::AttendanceMethod, attendance_status::AttendanceStatus,
    attendance_type::AttendanceType,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(FromRow))]
pub struct Attendance {
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub clock_in: Option<DateTime<Utc>>,
    pub clock_out: Option<DateTime<Utc>>,
    pub date: DateTime<Utc>,
    pub method: AttendanceMethod,
    pub status: AttendanceStatus,
    pub attendance_type: AttendanceType,
    pub lat: Option<f64>,
    pub long: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Attendance {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            organization_id: Uuid::new_v4(),
            clock_in: None,
            clock_out: None,
            date: now,
            method: AttendanceMethod::default(),
            status: AttendanceStatus::default(),
            attendance_type: AttendanceType::default(),
            lat: None,
            long: None,
            created_at: now,
            updated_at: now,
        }
    }
}
