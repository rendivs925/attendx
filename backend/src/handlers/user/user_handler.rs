use std::sync::Arc;

use crate::{
    services::user_service::{UserService, UserServiceError},
    utils::{
        http_utils::{handle_internal_error, handle_validation_error},
        locale_utils::get_lang,
    },
};
use actix_web::{HttpRequest, HttpResponse, web};
use shared::types::responses::api_response::ApiResponse;
use shared::{
    types::requests::auth::validation_request::ValidationRequest,
    utils::{locale_utils::Messages, validation_utils::validate_data},
};
use shared::{
    types::requests::user::update_user_request::UpdateUserRequest, utils::locale_utils::Namespace,
};

pub async fn get_all_users_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    match user_service.get_all_users().await {
        Ok(users) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::User, "fetch.all_success"),
            Some(users),
        )),
        Err(err) => handle_internal_error(err.to_message(&messages)),
    }
}

pub async fn get_user_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    email: web::Path<String>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    let validation_input = ValidationRequest {
        email: Some(email.to_string()),
        ..Default::default()
    };

    if let Err(errs) = validate_data(&validation_input, &messages) {
        let msg = messages.get_message(Namespace::Auth, "email.invalid");
        return handle_validation_error(errs, &msg);
    }

    match user_service.get_user(&email).await {
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::User, "fetch.success"),
            Some(user),
        )),
        Ok(None) => {
            let msg = UserServiceError::NotFound.to_message(&messages);
            HttpResponse::NotFound().json(ApiResponse::<()>::error(msg, None))
        }
        Err(err) => handle_internal_error(err.to_message(&messages)),
    }
}

pub async fn update_user_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    email: web::Path<String>,
    updated_user: web::Json<UpdateUserRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    let validation_input = ValidationRequest {
        email: Some(email.to_string()),
        ..Default::default()
    };

    if let Err(errs) = validate_data(&validation_input, &messages) {
        let msg = messages.get_message(Namespace::Auth, "email.invalid");
        return handle_validation_error(errs, &msg);
    }

    match user_service
        .update_user(&email, updated_user.into_inner())
        .await
    {
        Ok(updated) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::User, "update.success"),
            Some(updated),
        )),
        Err(err) => {
            let msg = err.to_message(&messages);
            match err {
                UserServiceError::NotFound => {
                    HttpResponse::NotFound().json(ApiResponse::<()>::error(msg, None))
                }
                _ => handle_internal_error(msg),
            }
        }
    }
}

pub async fn delete_user_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    email: web::Path<String>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    let validation_input = ValidationRequest {
        email: Some(email.to_string()),
        ..Default::default()
    };

    if let Err(errs) = validate_data(&validation_input, &messages) {
        let msg = messages.get_message(Namespace::Auth, "email.invalid");
        return handle_validation_error(errs, &msg);
    }

    match user_service.delete_user(&email).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::User, "delete.success"),
            None::<()>,
        )),
        Err(err) => {
            let msg = err.to_message(&messages);
            match err {
                UserServiceError::NotFound => {
                    HttpResponse::NotFound().json(ApiResponse::<()>::error(msg, None))
                }
                _ => handle_internal_error(msg),
            }
        }
    }
}
