use crate::repositories::attendance_repository::AttendanceRepository;
use chrono::Utc;
use shared::{
    models::attendance_model::Attendance,
    prelude::MessageLookup,
    types::{
        requests::attendance::{
            register_attendance_request::RegisterAttendanceRequest,
            update_attendance_request::UpdateAttendanceRequest,
        },
        responses::attendance_response::AttendanceResponse,
    },
    utils::locale_utils::Namespace,
};
use std::{fmt, sync::Arc};
use uuid::Uuid;

#[derive(Debug)]
pub enum AttendanceServiceError {
    NotFound,
    DuplicateAttendance,
    DbError(String),
    InvalidId(String),
}

impl AttendanceServiceError {
    pub fn to_message(&self, messages: &dyn MessageLookup) -> String {
        match self {
            AttendanceServiceError::NotFound => {
                messages.get_message(Namespace::Attendance, "fetch.not_found")
            }
            AttendanceServiceError::DuplicateAttendance => {
                messages.get_message(Namespace::Attendance, "create.duplicate")
            }
            AttendanceServiceError::DbError(_) => {
                messages.get_message(Namespace::Attendance, "db_error")
            }
            AttendanceServiceError::InvalidId(_) => {
                messages.get_message(Namespace::Attendance, "invalid_id")
            }
        }
    }
}

impl fmt::Display for AttendanceServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttendanceServiceError::NotFound => write!(f, "Attendance not found"),
            AttendanceServiceError::DuplicateAttendance => write!(f, "Duplicate attendance entry"),
            AttendanceServiceError::DbError(msg) => write!(f, "Database error: {}", msg),
            AttendanceServiceError::InvalidId(msg) => write!(f, "Invalid ID: {}", msg),
        }
    }
}

pub struct AttendanceService {
    pub attendance_repository: Arc<AttendanceRepository>,
}

impl AttendanceService {
    pub fn new(attendance_repository: Arc<AttendanceRepository>) -> Self {
        Self {
            attendance_repository,
        }
    }

    pub async fn create_attendance(
        &self,
        request: RegisterAttendanceRequest,
    ) -> Result<AttendanceResponse, AttendanceServiceError> {
        let user_id = Uuid::parse_str(&request.user_id)
            .map_err(|_| AttendanceServiceError::InvalidId("user_id".into()))?;
        let organization_id = Uuid::parse_str(&request.organization_id)
            .map_err(|_| AttendanceServiceError::InvalidId("organization_id".into()))?;

        let attendance = Attendance {
            user_id,
            organization_id,
            date: request.date,
            clock_in: request.clock_in,
            clock_out: request.clock_out,
            method: request.method.unwrap_or_else(Default::default),
            status: request.status.unwrap_or_else(Default::default),
            attendance_type: request.attendance_type.unwrap_or_else(Default::default),
            lat: request.lat,
            long: request.long,
            ..Default::default()
        };

        let created = self
            .attendance_repository
            .create_attendance(&attendance)
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?;

        Ok(AttendanceResponse::from(created))
    }

    pub async fn get_attendance_by_id(
        &self,
        id: &str,
    ) -> Result<Option<AttendanceResponse>, AttendanceServiceError> {
        let uuid =
            Uuid::parse_str(id).map_err(|e| AttendanceServiceError::InvalidId(e.to_string()))?;

        let attendance = self
            .attendance_repository
            .get_attendance_by_id(uuid)
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?;

        Ok(attendance.map(AttendanceResponse::from))
    }

    pub async fn get_all_attendances(
        &self,
    ) -> Result<Vec<AttendanceResponse>, AttendanceServiceError> {
        let result = self
            .attendance_repository
            .get_all_attendances()
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?;

        Ok(result.into_iter().map(AttendanceResponse::from).collect())
    }

    pub async fn update_attendance(
        &self,
        id: &str,
        req: UpdateAttendanceRequest,
    ) -> Result<AttendanceResponse, AttendanceServiceError> {
        let uuid =
            Uuid::parse_str(id).map_err(|e| AttendanceServiceError::InvalidId(e.to_string()))?;

        let mut existing = self
            .attendance_repository
            .get_attendance_by_id(uuid)
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?
            .ok_or(AttendanceServiceError::NotFound)?;

        existing.clock_in = req.clock_in;
        existing.clock_out = req.clock_out;
        existing.date = req.date.unwrap_or(existing.date);
        existing.method = req.method.unwrap_or_else(Default::default);
        existing.status = req.status.unwrap_or_else(Default::default);
        existing.attendance_type = req.attendance_type.unwrap_or_else(Default::default);
        if let Some(loc) = req.location {
            existing.lat = Some(loc.lat);
            existing.long = Some(loc.long);
        }
        existing.updated_at = Utc::now();

        let updated = self
            .attendance_repository
            .update_attendance(uuid, &existing)
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?;

        Ok(AttendanceResponse::from(updated))
    }

    pub async fn delete_attendance(&self, id: &str) -> Result<(), AttendanceServiceError> {
        let uuid =
            Uuid::parse_str(id).map_err(|e| AttendanceServiceError::InvalidId(e.to_string()))?;

        self.attendance_repository
            .delete_attendance(uuid)
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?;

        Ok(())
    }
}
