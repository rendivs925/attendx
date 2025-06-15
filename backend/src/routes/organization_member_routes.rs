use crate::config::cors::configure_cors;
use crate::handlers::rest::organization_member::organization_member_handler::{
    create_organization_member_handler, delete_organization_member_handler,
    get_all_organization_members_handler, get_organization_member_handler,
    update_organization_member_handler,
};
use crate::services::organization_member_service::OrganizationMemberService;
use actix_web::web;
use std::sync::Arc;

pub fn configure_organization_member_routes(
    cfg: &mut web::ServiceConfig,
    organization_member_service: web::Data<Arc<OrganizationMemberService>>,
) {
    cfg.service(
        web::scope("/organization-members")
            .app_data(organization_member_service)
            .route("/new", web::post().to(create_organization_member_handler))
            .route(
                "/all/{organization_id}",
                web::get().to(get_all_organization_members_handler),
            )
            .route(
                "/{org_id}/{member_id}",
                web::get().to(get_organization_member_handler),
            )
            .route(
                "/{org_id}/{member_id}",
                web::put().to(update_organization_member_handler),
            )
            .route(
                "/{org_id}/{member_id}",
                web::delete().to(delete_organization_member_handler),
            )
            .wrap(configure_cors()),
    );
}
