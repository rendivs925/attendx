use crate::graphql::context::GQLContext;
use crate::graphql::error::{AppError, graphql_error};
use crate::graphql::middleware::auth::extract_claims;
use crate::graphql::modules::user::model::UserObject;
use async_graphql::{Context, Object, Result};
use shared::types::requests::auth::register_request::RegisterRequest;
use shared::types::requests::user::update_user_request::UpdateUserRequest;

pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register_user(
        &self,
        ctx: &Context<'_>,
        mut input: RegisterRequest,
    ) -> Result<UserObject> {
        let claims = extract_claims(ctx)?;

        let user_id = claims
            .get("sub")
            .and_then(|v| v.as_str())
            .ok_or_else(|| graphql_error(AppError::Unauthorized))?;

        input.id = Some(user_id.to_string());

        let svc = &ctx.data::<GQLContext>()?.app_service.user_service;

        let user = svc
            .register_user(input)
            .await
            .map_err(|_| graphql_error(AppError::Conflict("Email already registered".into())))?;

        Ok(UserObject::from(user))
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        input: UpdateUserRequest,
    ) -> Result<UserObject> {
        let claims = extract_claims(ctx)?;
        let svc = &ctx.data::<GQLContext>()?.app_service.user_service;

        let updated = svc
            .update_user(&email, input)
            .await
            .map_err(|e| graphql_error(AppError::Internal(e.to_string())))?;

        Ok(UserObject::from(updated))
    }

    async fn delete_user(&self, ctx: &Context<'_>, email: String) -> Result<bool> {
        let claims = extract_claims(ctx)?;
        let svc = &ctx.data::<GQLContext>()?.app_service.user_service;

        svc.delete_user(&email)
            .await
            .map_err(|e| graphql_error(AppError::Internal(e.to_string())))?;

        Ok(true)
    }
}
