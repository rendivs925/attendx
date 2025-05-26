use actix_web::web::{Data, ServiceConfig};
use attendx_backend::{
    config::database::Database, routes::app_router::AppRouter, services::app_service::AppService,
};
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;
use std::sync::Arc;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    dotenv().ok();

    let _ = env_logger::try_init();

    let db = Arc::new(Database::new().await.expect("❌ Failed to init DB"));
    let app_service = AppService::new(db).await;
    let router = AppRouter::new(Data::new(app_service));

    let config = move |cfg: &mut ServiceConfig| {
        router.configure(cfg);
    };

    Ok(config.into())
}
