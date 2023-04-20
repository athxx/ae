pub mod errors;
pub mod gql;

use axum::{
    extract::Extension,
    http::{header, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use async_graphql_axum::GraphQLSubscription;

use core::service::schema::build_schema;

pub async fn router() -> Router {
    let schema = build_schema().await;
    Router::new()
        .route("/favicon.ico", get(errors::no_content))
        .route("/ping", get(errors::ping))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .route("/api", get(gql::graphiql).post(gql::graphql_handler))
        .layer(Extension(schema))
        .fallback(errors::not_found)
}
