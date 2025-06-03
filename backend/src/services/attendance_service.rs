use bson::{DateTime, oid::ObjectId};
use chrono::Utc;
use mongodb::bson::to_document;
use shared::{
    models::attendance_model::Attendance,
    types::{
        requests::attendance::{
            register_attendance_request::RegisterAttendanceRequest,
            update_attendance_request::UpdateAttendanceRequest,
        },
        responses::attendance_response::AttendanceResponse,
    },
    utils::locale_utils::{Messages, Namespace},
};
use std::{str::FromStr, sync::Arc};

use crate::repositories::attendance_repository::AttendanceRepository;

#[derive(Debug)]
pub enum AttendanceServiceError {
    NotFound,
    DuplicateAttendance,
    DbError(String),
    InvalidId(String),
}

impl AttendanceServiceError {
    pub fn to_message(&self, messages: &Messages) -> String {
        match self {
            AttendanceServiceError::NotFound => {
                messages.get_message(Namespace::Attendance, "fetch.not_found")
            }
            AttendanceServiceError::DuplicateAttendance => {
                messages.get_message(Namespace::Attendance, "create.duplicate")
            }
            AttendanceServiceError::DbError(e) => {
                eprintln!("Database error: {}", e);
                messages.get_message(Namespace::Attendance, "db_error")
            }
            AttendanceServiceError::InvalidId(e) => {
                messages.get_message(Namespace::Attendance, "invalid_id")
            }
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
        let user_id = ObjectId::from_str(&request.user_id)
            .map_err(|e| AttendanceServiceError::InvalidId(e.to_string()))?;
        let organization_id = ObjectId::from_str(&request.organization_id)
            .map_err(|e| AttendanceServiceError::InvalidId(e.to_string()))?;

        let now = Utc::now();
        let attendance = Attendance {
            _id: Some(ObjectId::new()),
            user_id,
            organization_id,
            attendance_type: request.attendance_type,
            status: request.status,
            clock_in: request.clock_in,
            clock_out: request.clock_out,
            method: request.method,
            location: request.location,
            created_at: now,
            updated_at: now,
        };

        let created_attendance = self
            .attendance_repository
            .create_attendance(&attendance)
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?;

        Ok(AttendanceResponse::from(created_attendance))
    }

    pub async fn get_attendance_by_id(
        &self,
        attendance_id_str: &str,
    ) -> Result<Option<AttendanceResponse>, AttendanceServiceError> {
        let attendance_id = ObjectId::from_str(attendance_id_str)
            .map_err(|e| AttendanceServiceError::InvalidId(e.to_string()))?;

        let attendance = self
            .attendance_repository
            .get_attendance_by_id(&attendance_id)
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?;

        Ok(attendance.map(AttendanceResponse::from))
    }

    pub async fn get_all_attendances_for_user_in_org(
        &self,
        user_id_str: &str,
        organization_id_str: &str,
    ) -> Result<Vec<AttendanceResponse>, AttendanceServiceError> {
        let user_id = ObjectId::from_str(user_id_str)
            .map_err(|e| AttendanceServiceError::InvalidId(e.to_string()))?;
        let organization_id = ObjectId::from_str(organization_id_str)
            .map_err(|e| AttendanceServiceError::InvalidId(e.to_string()))?;

        let attendances = self
            .attendance_repository
            .get_all_attendances_for_user_in_org(&user_id, &organization_id)
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?;

        Ok(attendances
            .into_iter()
            .map(AttendanceResponse::from)
            .collect())
    }

    pub async fn get_all_attendances_for_org(
        &self,
        organization_id_str: &str,
    ) -> Result<Vec<AttendanceResponse>, AttendanceServiceError> {
        let organization_id = ObjectId::from_str(organization_id_str)
            .map_err(|e| AttendanceServiceError::InvalidId(e.to_string()))?;

        let attendances = self
            .attendance_repository
            .get_all_attendances_for_org(&organization_id)
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?;

        Ok(attendances
            .into_iter()
            .map(AttendanceResponse::from)
            .collect())
    }

    pub async fn update_attendance(
        &self,
        attendance_id_str: &str,
        updated_request: UpdateAttendanceRequest,
    ) -> Result<AttendanceResponse, AttendanceServiceError> {
        let attendance_id = ObjectId::from_str(attendance_id_str)
            .map_err(|e| AttendanceServiceError::InvalidId(e.to_string()))?;

        self.get_attendance_by_id(attendance_id_str)
            .await?
            .ok_or(AttendanceServiceError::NotFound)?;

        let mut update_doc = to_document(&updated_request).map_err(|e| {
            AttendanceServiceError::DbError(format!(
                "Failed to convert update request to document: {}",
                e
            ))
        })?;

        update_doc.insert("updated_at", DateTime::now());

        let updated_attendance = self
            .attendance_repository
            .update_attendance(&attendance_id, update_doc)
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?
            .ok_or(AttendanceServiceError::NotFound)?;

        Ok(AttendanceResponse::from(updated_attendance))
    }

    pub async fn delete_attendance(
        &self,
        attendance_id_str: &str,
    ) -> Result<(), AttendanceServiceError> {
        let attendance_id = ObjectId::from_str(attendance_id_str)
            .map_err(|e| AttendanceServiceError::InvalidId(e.to_string()))?;

        self.attendance_repository
            .delete_attendance(&attendance_id)
            .await
            .map_err(|e| AttendanceServiceError::DbError(e.to_string()))?;

        Ok(())
    }
}
