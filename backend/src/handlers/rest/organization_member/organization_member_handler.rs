use actix_web::{HttpRequest, HttpResponse, web};
use log::{info, warn};
use shared::prelude::*;
use std::sync::Arc;

use shared::{
    types::{
        requests::{
            auth::validation_request::ValidationRequest,
            organization_member::{
                register_organization_member_request::RegisterOrganizationMemberRequest,
                update_organization_member_request::UpdateOrganizationMemberRequest,
            },
        },
        responses::api_response::ApiResponse,
    },
    utils::{
        locale_utils::{Messages, Namespace},
        validation_utils::validate_data,
    },
};

use crate::{
    services::organization_member_service::{
        OrganizationMemberService, OrganizationMemberServiceError,
    },
    utils::{
        http_utils::{handle_internal_error, handle_validation_error},
        locale_utils::get_lang,
    },
};

pub async fn create_organization_member_handler(
    req: HttpRequest,
    organization_member_service: web::Data<Arc<OrganizationMemberService>>,
    member_json: web::Json<RegisterOrganizationMemberRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let new_member = member_json.into_inner();

    let organization_id = &new_member.organization_id.clone();

    info!("Creating member for organization_id: {}", organization_id);

    let email_to_validate = new_member.identifiers.get("email").cloned();
    let validation_input = ValidationRequest {
        email: email_to_validate,
        ..Default::default()
    };

    if let Err(errs) = validate_data(&validation_input, &messages) {
        let msg = messages.get_message(Namespace::Auth, "email.invalid");
        warn!(
            "Validation failed when creating member for org_id {}: {:?}",
            organization_id, errs
        );
        return handle_validation_error(errs, &msg);
    }

    match organization_member_service
        .create_member(organization_id, new_member)
        .await
    {
        Ok(created_member) => {
            info!(
                "Successfully created member for organization_id: {}",
                organization_id
            );
            HttpResponse::Created().json(ApiResponse::success(
                messages.get_message(Namespace::OrganizationMember, "create.success"),
                Some(created_member),
            ))
        }
        Err(err) => {
            warn!(
                "Failed to create member for organization_id {}: {:?}",
                organization_id, err
            );
            let msg = err.to_message(&messages);
            handle_internal_error(msg)
        }
    }
}

pub async fn get_organization_member_handler(
    req: HttpRequest,
    organization_member_service: web::Data<Arc<OrganizationMemberService>>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let (organization_id, member_id) = path.into_inner();

    info!(
        "Fetching member_id {} for organization_id {}",
        member_id, organization_id
    );

    match organization_member_service
        .get_member_by_id(&organization_id, &member_id)
        .await
    {
        Ok(Some(member)) => {
            info!(
                "Found member_id {} for organization_id {}",
                member_id, organization_id
            );
            HttpResponse::Ok().json(ApiResponse::success(
                messages.get_message(Namespace::OrganizationMember, "fetch.success"),
                Some(member),
            ))
        }
        Ok(None) => {
            warn!(
                "Member_id {} not found in organization_id {}",
                member_id, organization_id
            );
            let msg = OrganizationMemberServiceError::NotFound.to_message(&messages);
            HttpResponse::NotFound().json(ApiResponse::<()>::error(msg, None))
        }
        Err(err) => {
            warn!(
                "Failed to fetch member_id {} for organization_id {}: {:?}",
                member_id, organization_id, err
            );
            handle_internal_error(err.to_message(&messages))
        }
    }
}

pub async fn get_all_organization_members_handler(
    req: HttpRequest,
    organization_member_service: web::Data<Arc<OrganizationMemberService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let organization_id = path.into_inner();

    info!(
        "Fetching all members for organization_id {}",
        organization_id
    );

    match organization_member_service
        .get_all_members(&organization_id)
        .await
    {
        Ok(members) => {
            info!(
                "Fetched {} members for organization_id {}",
                members.len(),
                organization_id
            );
            HttpResponse::Ok().json(ApiResponse::success(
                messages.get_message(Namespace::OrganizationMember, "fetch.all_success"),
                Some(members),
            ))
        }
        Err(err) => {
            warn!(
                "Failed to fetch members for organization_id {}: {:?}",
                organization_id, err
            );
            handle_internal_error(err.to_message(&messages))
        }
    }
}

pub async fn update_organization_member_handler(
    req: HttpRequest,
    organization_member_service: web::Data<Arc<OrganizationMemberService>>,
    path: web::Path<(String, String)>,
    member_json: web::Json<UpdateOrganizationMemberRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let (organization_id, member_id) = path.into_inner();
    let updated_member = member_json.into_inner();

    info!(
        "Updating member_id {} for organization_id {}",
        member_id, organization_id
    );

    let email_to_validate = updated_member
        .identifiers
        .as_ref()
        .and_then(|ids| ids.get("email").cloned());

    let validation_input = ValidationRequest {
        email: email_to_validate,
        ..Default::default()
    };

    if let Err(errs) = validate_data(&validation_input, &messages) {
        let msg = messages.get_message(Namespace::Auth, "email.invalid");
        warn!(
            "Validation failed when updating member_id {} for org_id {}: {:?}",
            member_id, organization_id, errs
        );
        return handle_validation_error(errs, &msg);
    }

    match organization_member_service
        .update_member(&organization_id, &member_id, updated_member)
        .await
    {
        Ok(member) => {
            info!(
                "Successfully updated member_id {} for organization_id {}",
                member_id, organization_id
            );
            HttpResponse::Ok().json(ApiResponse::success(
                messages.get_message(Namespace::OrganizationMember, "update.success"),
                Some(member),
            ))
        }
        Err(err) => {
            warn!(
                "Failed to update member_id {} for org_id {}: {:?}",
                member_id, organization_id, err
            );
            let msg = err.to_message(&messages);
            match err {
                OrganizationMemberServiceError::NotFound => {
                    HttpResponse::NotFound().json(ApiResponse::<()>::error(msg, None))
                }
                _ => handle_internal_error(msg),
            }
        }
    }
}

pub async fn delete_organization_member_handler(
    req: HttpRequest,
    organization_member_service: web::Data<Arc<OrganizationMemberService>>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let (organization_id, member_id) = path.into_inner();

    info!(
        "Deleting member_id {} for organization_id {}",
        member_id, organization_id
    );

    match organization_member_service
        .delete_member(&organization_id, &member_id)
        .await
    {
        Ok(_) => {
            info!(
                "Deleted member_id {} for organization_id {}",
                member_id, organization_id
            );
            HttpResponse::Ok().json(ApiResponse::success(
                messages.get_message(Namespace::OrganizationMember, "delete.success"),
                None::<()>,
            ))
        }
        Err(err) => {
            warn!(
                "Failed to delete member_id {} for org_id {}: {:?}",
                member_id, organization_id, err
            );
            let msg = err.to_message(&messages);
            match err {
                OrganizationMemberServiceError::NotFound => {
                    HttpResponse::NotFound().json(ApiResponse::<()>::error(msg, None))
                }
                _ => handle_internal_error(msg),
            }
        }
    }
}
