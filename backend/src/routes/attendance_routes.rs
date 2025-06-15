use crate::config::cors::configure_cors;
use crate::handlers::rest::attendance::attendance_handler::{
    create_attendance_handler, delete_attendance_handler, get_all_attendances_handler,
    get_attendance_handler, update_attendance_handler,
};
use crate::handlers::ws::attendance_ws::attendance_ws_handler;
use crate::services::attendance_service::AttendanceService;
use actix_web::web;
use std::sync::Arc;

pub fn configure_attendance_routes(
    cfg: &mut web::ServiceConfig,
    attendance_service: web::Data<Arc<AttendanceService>>,
) {
    cfg.service(
        web::scope("/attendances")
            .app_data(attendance_service.clone())
            .route("/new", web::post().to(create_attendance_handler))
            .route("/all", web::get().to(get_all_attendances_handler))
            .route("/{attendance_id}", web::get().to(get_attendance_handler))
            .route("/{attendance_id}", web::put().to(update_attendance_handler))
            .route(
                "/{attendance_id}",
                web::delete().to(delete_attendance_handler),
            )
            .wrap(configure_cors()),
    );

    cfg.service(
        web::resource("/ws/attendance")
            .app_data(attendance_service.clone())
            .route(web::get().to(attendance_ws_handler)),
    );
}
