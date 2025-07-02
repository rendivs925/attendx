use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Default, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "attendance_type"))]
pub enum AttendanceType {
    #[default]
    SingleMark,
    DoubleMark,
}
