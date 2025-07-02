use crate::services::app_service::AppService;
use actix_web::web;

use super::{
    attendance_routes::configure_attendance_routes, auth_routes::configure_auth_routes,
    organization_routes::configure_organization_routes, user_routes::configure_user_routes,
};

#[derive(Clone)]
pub struct AppRouter {
    app_service: web::Data<AppService>,
}

impl AppRouter {
    pub fn new(app_service: web::Data<AppService>) -> Self {
        Self { app_service }
    }

    pub fn configure(&self, cfg: &mut web::ServiceConfig) {
        let user_service = web::Data::new(self.app_service.user_service.clone());
        let org_service = web::Data::new(self.app_service.organization_service.clone());
        let attendance_service = web::Data::new(self.app_service.attendance_service.clone());

        cfg.service(web::scope("/api").configure(|cfg| {
            configure_user_routes(cfg, user_service.clone());
            configure_auth_routes(cfg, user_service);
            configure_organization_routes(cfg, org_service);
            configure_attendance_routes(cfg, attendance_service);
        }));
    }
}
