use crate::{
    config::database::Database,
    repositories::{
        attendance_repository::AttendanceRepository,
        organization_repository::OrganizationRepository, user_repository::UserRepository,
    },
};
use std::sync::Arc;

pub struct AppRepository {
    pub user_repository: Arc<UserRepository>,
    pub organization_repository: Arc<OrganizationRepository>,
    pub attendance_repository: Arc<AttendanceRepository>,
}

impl AppRepository {
    pub fn new(db: Arc<Database>) -> Self {
        let user_repository = Arc::new(UserRepository::new(db.pool.clone()));
        let organization_repository = Arc::new(OrganizationRepository::new(db.pool.clone()));
        let attendance_repository = Arc::new(AttendanceRepository::new(db.pool.clone()));

        Self {
            user_repository,
            organization_repository,
            attendance_repository,
        }
    }
}
