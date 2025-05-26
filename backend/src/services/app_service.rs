use crate::{
    config::database::Database,
    repositories::{
        organization_repository::OrganizationRepository, user_repository::UserRepository,
    },
    services::{organization_service::OrganizationService, user_service::UserService},
};
use std::sync::Arc;

pub struct AppService {
    pub user_service: Arc<UserService>,
    pub organization_service: Arc<OrganizationService>,
}

impl AppService {
    pub async fn new(db: Arc<Database>) -> Self {
        let user_repository = UserRepository::new(db.clone())
            .await
            .expect("❌ Failed to initialize UserRepository");
        let organization_repository = OrganizationRepository::new(db.clone())
            .await
            .expect("❌ Failed to initialize OrganizationRepository");

        let user_service = Arc::new(UserService::new(Arc::new(user_repository)));
        let organization_service =
            Arc::new(OrganizationService::new(Arc::new(organization_repository)));

        Self {
            user_service,
            organization_service,
        }
    }
}
