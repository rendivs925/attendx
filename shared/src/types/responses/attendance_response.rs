use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    models::attendance_model::Attendance,
    types::models::attendance::{
        attendance_method::AttendanceMethod, attendance_status::AttendanceStatus,
        attendance_type::AttendanceType, geolocation::GeoLocation,
    },
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttendanceResponse {
    pub id: String,
    pub user_id: String,
    pub organization_id: String,
    pub attendance_type: AttendanceType,
    pub status: AttendanceStatus,
    pub clock_in: Option<DateTime<Utc>>,
    pub clock_out: Option<DateTime<Utc>>,
    pub method: AttendanceMethod,
    pub location: Option<GeoLocation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Attendance> for AttendanceResponse {
    fn from(attendance: Attendance) -> Self {
        Self {
            id: attendance
                ._id
                .map_or_else(String::default, |id| id.to_hex()),
            user_id: attendance.user_id.to_hex(),
            organization_id: attendance.organization_id.to_hex(),
            attendance_type: attendance.attendance_type,
            status: attendance.status,
            clock_in: attendance.clock_in,
            clock_out: attendance.clock_out,
            method: attendance.method,
            location: attendance.location,
            created_at: attendance.created_at,
            updated_at: attendance.updated_at,
        }
    }
}
