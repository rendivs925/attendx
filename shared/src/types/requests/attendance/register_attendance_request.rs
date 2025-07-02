use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::models::attendance::{
    attendance_method::AttendanceMethod, attendance_status::AttendanceStatus,
    attendance_type::AttendanceType, geolocation::GeoLocation,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAttendanceRequest {
    pub user_id: String,
    pub organization_id: String,
    pub date: DateTime<Utc>,
    pub clock_in: Option<DateTime<Utc>>,
    pub clock_out: Option<DateTime<Utc>>,
    pub method: Option<AttendanceMethod>,
    pub status: Option<AttendanceStatus>,
    pub attendance_type: Option<AttendanceType>,
    pub lat: Option<f64>,
    pub long: Option<f64>,
}
