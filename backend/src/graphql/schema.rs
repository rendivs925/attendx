use crate::graphql::{
    context::GQLContext,
    modules::user::{UserMutation, UserQuery},
};
use async_graphql::{EmptySubscription, Schema};

pub type AppSchema = Schema<UserQuery, UserMutation, EmptySubscription>;

pub fn create_schema(ctx: GQLContext) -> AppSchema {
    Schema::build(UserQuery, UserMutation, EmptySubscription)
        .data(ctx)
        .finish()
}
