use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[cfg(feature = "backend")]
use sqlx::Type;

#[derive(Debug, Default, Clone, PartialEq, Display, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "attendance_status"))]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum AttendanceStatus {
    Present,
    #[default]
    Absent,
    Late,
    Excused,
}
