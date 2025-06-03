use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Display, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Permission {
    MarkAttendance,
    ViewAttendance,
    ManageUsers,
}
