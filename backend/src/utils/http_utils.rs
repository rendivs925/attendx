use actix_web::HttpResponse;
use serde_json::json;

use shared::types::responses::api_response::{ApiResponse, ErrorDetails};
use validator::ValidationErrors;

pub fn handle_validation_error(errors: ValidationErrors, msg: &str) -> HttpResponse {
    let error_details = ErrorDetails {
        details: Some(json!(&errors)),
    };
    HttpResponse::BadRequest().json(ApiResponse::<()>::error(msg, Some(error_details)))
}

pub fn handle_internal_error(err: impl ToString) -> HttpResponse {
    HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err.to_string(), None))
}
