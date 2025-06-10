use crate::{
    services::attendance_service::{AttendanceService, AttendanceServiceError},
    utils::{http_utils::handle_internal_error, locale_utils::get_lang},
};
use actix_web::{HttpRequest, HttpResponse, web};
use shared::{
    types::{
        requests::attendance::{
            register_attendance_request::RegisterAttendanceRequest,
            update_attendance_request::UpdateAttendanceRequest,
        },
        responses::api_response::ApiResponse,
    },
    utils::locale_utils::{Messages, Namespace},
};
use std::sync::Arc;

pub async fn create_attendance_handler(
    req: HttpRequest,
    attendance_service: web::Data<Arc<AttendanceService>>,
    payload: web::Json<RegisterAttendanceRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let data = payload.into_inner();

    match attendance_service.create_attendance(data).await {
        Ok(attendance) => HttpResponse::Created().json(ApiResponse::success(
            messages.get_message(Namespace::Attendance, "create.success"),
            Some(attendance),
        )),
        Err(e) => handle_internal_error(e.to_message(&messages)),
    }
}

pub async fn get_attendance_handler(
    req: HttpRequest,
    attendance_service: web::Data<Arc<AttendanceService>>,
    id: web::Path<String>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let id_str = id.as_str();

    match attendance_service.get_attendance_by_id(id_str).await {
        Ok(Some(att)) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::Attendance, "fetch.success"),
            Some(att),
        )),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            AttendanceServiceError::NotFound.to_message(&messages),
            None,
        )),
        Err(e) => handle_internal_error(e.to_message(&messages)),
    }
}

pub async fn get_all_attendances_handler(
    req: HttpRequest,
    attendance_service: web::Data<Arc<AttendanceService>>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    match attendance_service.get_all_attendances().await {
        Ok(list) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::Attendance, "fetch.all_success"),
            Some(list),
        )),
        Err(e) => handle_internal_error(e.to_message(&messages)),
    }
}

pub async fn update_attendance_handler(
    req: HttpRequest,
    attendance_service: web::Data<Arc<AttendanceService>>,
    id: web::Path<String>,
    payload: web::Json<UpdateAttendanceRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let id_str = id.as_str();

    match attendance_service
        .update_attendance(id_str, payload.into_inner())
        .await
    {
        Ok(att) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::Attendance, "update.success"),
            Some(att),
        )),
        Err(AttendanceServiceError::NotFound) => HttpResponse::NotFound().json(
            ApiResponse::<()>::error(AttendanceServiceError::NotFound.to_message(&messages), None),
        ),
        Err(e) => handle_internal_error(e.to_message(&messages)),
    }
}

pub async fn delete_attendance_handler(
    req: HttpRequest,
    attendance_service: web::Data<Arc<AttendanceService>>,
    id: web::Path<String>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let id_str = id.as_str();

    match attendance_service.delete_attendance(id_str).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::Attendance, "delete.success"),
            None::<()>,
        )),
        Err(AttendanceServiceError::NotFound) => HttpResponse::NotFound().json(
            ApiResponse::<()>::error(AttendanceServiceError::NotFound.to_message(&messages), None),
        ),
        Err(e) => handle_internal_error(e.to_message(&messages)),
    }
}
