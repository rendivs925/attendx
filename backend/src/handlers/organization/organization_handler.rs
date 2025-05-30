use actix_web::{HttpRequest, HttpResponse, web};
use std::sync::Arc;

use shared::{
    models::organization_model::Organization,
    types::{
        requests::{
            auth::validation_request::ValidationRequest,
            organization::register_organization_request::RegisterOrganizationRequest,
        },
        responses::api_response::ApiResponse,
    },
    utils::{
        locale_utils::{Messages, Namespace},
        validation_utils::validate_data,
    },
};

use crate::{
    services::organization_service::{OrganizationService, OrganizationServiceError},
    utils::{
        http_utils::{handle_internal_error, handle_validation_error},
        locale_utils::get_lang,
    },
};

pub async fn create_organization_handler(
    req: HttpRequest,
    organization_service: web::Data<Arc<OrganizationService>>,
    organization_json: web::Json<RegisterOrganizationRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let new_organization = organization_json.into_inner();

    let validation_input = ValidationRequest {
        email: Some(new_organization.email.clone()),
        ..Default::default()
    };

    if let Err(errs) = validate_data(&validation_input, &messages) {
        let msg = messages.get_message(Namespace::Auth, "email.invalid");
        return handle_validation_error(errs, &msg);
    }

    match organization_service
        .create_organization(new_organization)
        .await
    {
        Ok(new_org) => HttpResponse::Created().json(ApiResponse::success(
            messages.get_message(Namespace::Organization, "create.success"),
            Some(new_org),
        )),
        Err(err) => {
            let msg = err.to_message(&messages);
            match err {
                OrganizationServiceError::DuplicateEmail => {
                    HttpResponse::Conflict().json(ApiResponse::<()>::error(msg, None))
                }
                _ => handle_internal_error(msg),
            }
        }
    }
}

pub async fn get_organization_handler(
    req: HttpRequest,
    organization_service: web::Data<Arc<OrganizationService>>,
    org_id: web::Path<String>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    match organization_service.get_organization_by_id(&org_id).await {
        Ok(Some(org)) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::Organization, "fetch.success"),
            Some(org),
        )),
        Ok(None) => {
            let msg = OrganizationServiceError::NotFound.to_message(&messages);
            HttpResponse::NotFound().json(ApiResponse::<()>::error(msg, None))
        }
        Err(err) => handle_internal_error(err.to_message(&messages)),
    }
}

pub async fn get_all_organizations_handler(
    req: HttpRequest,
    organization_service: web::Data<Arc<OrganizationService>>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    match organization_service.get_all_organizations().await {
        Ok(orgs) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::Organization, "fetch.all_success"),
            Some(orgs),
        )),
        Err(err) => handle_internal_error(err.to_message(&messages)),
    }
}

pub async fn update_organization_handler(
    req: HttpRequest,
    organization_service: web::Data<Arc<OrganizationService>>,
    org_id: web::Path<String>,
    organization_json: web::Json<Organization>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let updated_organization = organization_json.into_inner();

    let validation_input = ValidationRequest {
        email: Some(updated_organization.email.clone()),
        ..Default::default()
    };

    if let Err(errs) = validate_data(&validation_input, &messages) {
        let msg = messages.get_message(Namespace::Auth, "email.invalid");
        return handle_validation_error(errs, &msg);
    }

    match organization_service
        .update_organization(&org_id, updated_organization)
        .await
    {
        Ok(updated_org) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::Organization, "update.success"),
            Some(updated_org),
        )),
        Err(err) => {
            let msg = err.to_message(&messages);
            match err {
                OrganizationServiceError::NotFound => {
                    HttpResponse::NotFound().json(ApiResponse::<()>::error(msg, None))
                }
                _ => handle_internal_error(msg),
            }
        }
    }
}

pub async fn delete_organization_handler(
    req: HttpRequest,
    organization_service: web::Data<Arc<OrganizationService>>,
    org_id: web::Path<String>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    match organization_service.delete_organization(&org_id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_message(Namespace::Organization, "delete.success"),
            None::<()>,
        )),
        Err(err) => {
            let msg = err.to_message(&messages);
            match err {
                OrganizationServiceError::NotFound => {
                    HttpResponse::NotFound().json(ApiResponse::<()>::error(msg, None))
                }
                _ => handle_internal_error(msg),
            }
        }
    }
}
