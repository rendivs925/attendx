use std::sync::Arc;

use actix_cors::Cors;
use actix_web::HttpRequest;
use actix_web::web;
use actix_web::{HttpResponse, web::Data};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use attendx_backend::graphql::context::GQLContext;
use attendx_backend::{
    config::database::Database,
    graphql::schema::{AppSchema, create_schema},
    repositories::app_repository::AppRepository,
    services::app_service::AppService,
};
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;

use async_graphql::Request;

pub async fn graphql_handler(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut inner_req = gql_req.into_inner();

    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = header_str.strip_prefix("Bearer ") {
                inner_req = inner_req.data(token.to_string());
            }
        }
    }

    schema.execute(inner_req).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[shuttle_runtime::main]
async fn main()
-> ShuttleActixWeb<impl FnOnce(&mut actix_web::web::ServiceConfig) + Clone + Send + 'static> {
    dotenv().ok();
    let _ = env_logger::try_init();

    let db = Arc::new(Database::new().await.expect("Failed to connect to DB"));
    let app_repository = Arc::new(AppRepository::new(db.clone()));
    let app_service = Arc::new(AppService::new(app_repository).await);

    let gql_ctx = GQLContext { app_service };
    let schema = Data::new(create_schema(gql_ctx));

    let config = move |cfg: &mut web::ServiceConfig| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        cfg.app_data(schema.clone());

        cfg.service(
            web::resource("/graphql")
                .route(web::post().to(graphql_handler))
                .wrap(cors),
        );

        cfg.service(web::resource("/playground").route(web::get().to(graphql_playground)));
    };

    Ok(config.into())
}
