use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    requests::attendance::{
        register_attendance_request::RegisterAttendanceRequest,
        update_attendance_request::UpdateAttendanceRequest,
    },
    responses::api_response::ApiResponse,
};

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum AttendanceWsMessage {
    Create(RegisterAttendanceRequest),
    Read(String),
    ReadAll,
    Update {
        id: String,
        payload: UpdateAttendanceRequest,
    },
    Delete(String),
}

#[derive(Serialize, Debug, Deserialize)]
pub struct AttendanceWsResponse {
    pub response: ApiResponse<Value>,
}
