use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::models::attendance::{
    attendance_method::AttendanceMethod, attendance_status::AttendanceStatus,
    attendance_type::AttendanceType, geolocation::GeoLocation,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAttendanceRequest {
    pub attendance_type: Option<AttendanceType>,
    pub status: Option<AttendanceStatus>,
    pub clock_in: Option<DateTime<Utc>>,
    pub clock_out: Option<DateTime<Utc>>,
    pub method: Option<AttendanceMethod>,
    pub location: Option<GeoLocation>,
}
