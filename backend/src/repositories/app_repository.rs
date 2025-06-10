use crate::{
    config::database::Database,
    repositories::{
        attendance_repository::AttendanceRepository,
        organization_member_repository::OrganizationMemberRepository,
        organization_repository::OrganizationRepository, user_repository::UserRepository,
    },
};
use std::sync::Arc;

pub struct AppRepository {
    pub user_repository: Arc<UserRepository>,
    pub organization_repository: Arc<OrganizationRepository>,
    pub organization_member_repository: Arc<OrganizationMemberRepository>,
    pub attendance_repository: Arc<AttendanceRepository>,
}

impl AppRepository {
    pub async fn new(db: Arc<Database>) -> Self {
        let user_repository = UserRepository::new(db.clone())
            .await
            .expect("Failed to initialize UserRepository");

        let organization_repository = OrganizationRepository::new(db.clone())
            .await
            .expect("Failed to initialize OrganizationRepository");

        let organization_member_repository = OrganizationMemberRepository::new(db.clone())
            .await
            .expect("Failed to initialize OrganizationMemberRepository");

        let attendance_repository = AttendanceRepository::new(db.clone())
            .await
            .expect("Failed to initialize AttendanceRepository");

        Self {
            user_repository: Arc::new(user_repository),
            organization_repository: Arc::new(organization_repository),
            organization_member_repository: Arc::new(organization_member_repository),
            attendance_repository: Arc::new(attendance_repository),
        }
    }
}
