use crate::services::app_service::AppService;
use std::sync::Arc;

pub struct GQLContext {
    pub app_service: Arc<AppService>,
}

impl GQLContext {
    pub fn new(app_service: Arc<AppService>) -> Self {
        Self { app_service }
    }
}
