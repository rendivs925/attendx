use crate::graphql::context::GQLContext;
use crate::graphql::error::{AppError, graphql_error};
use crate::graphql::middleware::auth::extract_claims;
use crate::graphql::modules::user::model::UserObject;
use async_graphql::{Context, Object, Result};
use serde_json::Value;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserObject>> {
        let claims = extract_claims(ctx)?;
        let svc = &ctx.data::<GQLContext>()?.app_service.user_service;

        let users = svc
            .get_all_users()
            .await
            .map_err(|e| graphql_error(AppError::Internal(e.to_string())))?;

        Ok(users.into_iter().map(UserObject::from).collect())
    }

    async fn user_by_email(&self, ctx: &Context<'_>, email: String) -> Result<Option<UserObject>> {
        let claims = extract_claims(ctx)?;
        let svc = &ctx.data::<GQLContext>()?.app_service.user_service;

        let user = svc
            .get_user(&email)
            .await
            .map_err(|_| graphql_error(AppError::NotFound("User not found".into())))?;

        Ok(user.map(UserObject::from))
    }
}
