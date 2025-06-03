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
    pub attendance_type: AttendanceType,
    pub status: AttendanceStatus,
    pub clock_in: Option<DateTime<Utc>>,
    pub clock_out: Option<DateTime<Utc>>,
    pub method: AttendanceMethod,
    pub location: Option<GeoLocation>,
}
