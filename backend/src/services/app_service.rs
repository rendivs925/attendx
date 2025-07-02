use crate::{
    repositories::app_repository::AppRepository,
    services::{
        attendance_service::AttendanceService, organization_service::OrganizationService,
        user_service::UserService,
    },
};
use std::sync::Arc;

pub struct AppService {
    pub user_service: Arc<UserService>,
    pub organization_service: Arc<OrganizationService>,
    pub attendance_service: Arc<AttendanceService>,
}

impl AppService {
    pub async fn new(repo: Arc<AppRepository>) -> Self {
        let user_service = Arc::new(UserService::new(repo.user_repository.clone()));
        let organization_service = Arc::new(OrganizationService::new(
            repo.organization_repository.clone(),
        ));
        let attendance_service =
            Arc::new(AttendanceService::new(repo.attendance_repository.clone()));

        Self {
            user_service,
            organization_service,
            attendance_service,
        }
    }
}
