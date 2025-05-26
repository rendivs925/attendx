use actix_web::{HttpRequest, HttpResponse, web};
use log::info;
use shared::types::requests::auth::login_request::LoginRequest;
use shared::types::requests::auth::register_request::RegisterRequest;
use shared::types::responses::api_response::ApiResponse;
use shared::types::responses::user_response::UserResponse;
use shared::utils::validation_utils::validate_login;
use shared::{
    types::requests::auth::validation_request::ValidationRequest,
    utils::{locale_utils::Messages, validation_utils::validate_data},
};
use std::sync::Arc;

use crate::utils::locale_utils::get_lang;
use crate::{
    constants::COOKIE_NAME,
    services::user_service::{UserService, UserServiceError},
    utils::{auth_utils::generate_cookie, http_utils::handle_validation_error},
};

pub async fn register_user_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    new_user: web::Json<RegisterRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let data = new_user.into_inner();

    let validation_data = ValidationRequest {
        name: Some(data.name.clone()),
        email: Some(data.email.clone()),
        password: Some(data.password.clone()),
        password_confirmation: Some(data.password_confirmation.clone()),
    };

    if let Err(errs) = validate_data(&validation_data, &messages) {
        let err_msg =
            messages.get_auth_message("register.invalid_data", "Invalid registration data");
        return handle_validation_error(errs, &err_msg);
    }

    match user_service.register_user(data).await {
        Ok(user) => {
            let user_response = UserResponse::from(user);
            HttpResponse::Created().json(ApiResponse::success(
                messages.get_auth_message("register.success", "User successfully created."),
                user_response,
            ))
        }
        Err(UserServiceError::DuplicateEmail) => HttpResponse::Conflict().json(
            ApiResponse::<()>::error(UserServiceError::DuplicateEmail.to_message(&messages), None),
        ),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error(e.to_message(&messages), None)),
    }
}

pub async fn jwt_login_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    credentials: web::Json<LoginRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let data = credentials.into_inner();

    if let Err(errs) = validate_login(&data.email, &data.password, &messages) {
        let err_msg =
            messages.get_auth_message("login.invalid_credentials", "Invalid login credentials");
        return handle_validation_error(errs, &err_msg);
    }

    match user_service
        .authenticate_user(&data.email, &data.password)
        .await
    {
        Ok((user, token)) => {
            info!("User {} successfully logged in.", data.email);
            let cookie = generate_cookie(token);
            let user_response = UserResponse::from(user);

            HttpResponse::Ok().cookie(cookie).json(ApiResponse::success(
                messages.get_auth_message("login.success", "Login successful"),
                user_response,
            ))
        }
        Err(UserServiceError::InvalidCredentials | UserServiceError::NotFound) => {
            HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                UserServiceError::InvalidCredentials.to_message(&messages),
                None,
            ))
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error(e.to_message(&messages), None)),
    }
}

pub async fn logout_user_handler(req: HttpRequest) -> HttpResponse {
    use actix_web::cookie::{Cookie, SameSite, time::Duration};

    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    let expired = Cookie::build(&*COOKIE_NAME, "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .path("/")
        .max_age(Duration::new(0, 0))
        .finish();

    HttpResponse::Ok()
        .cookie(expired)
        .json(ApiResponse::success(
            messages.get_auth_message("logout.success", "Logged out successfully."),
            None::<()>,
        ))
}
