use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Default, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum AttendanceMethod {
    QRCode,
    FacialRecognition,
    NFC,
    #[default]
    Manual,
    GPS,
    Biometric,
}
