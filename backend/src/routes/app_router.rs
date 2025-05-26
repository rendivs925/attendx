use crate::services::app_service::AppService;
use actix_web::web;

use super::{
    auth_routes::configure_auth_routes, organization_routes::configure_organization_routes,
    user_routes::configure_user_routes,
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
        configure_user_routes(cfg, web::Data::new(self.app_service.user_service.clone()));
        configure_auth_routes(cfg, web::Data::new(self.app_service.user_service.clone()));
        configure_organization_routes(
            cfg,
            web::Data::new(self.app_service.organization_service.clone()),
        );
    }
}
