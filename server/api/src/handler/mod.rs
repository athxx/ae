mod errors;

use axum::{
    http::{header, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

pub async fn router() -> Router {
    Router::new()
        .route("/", get(errors::ping))
        .route("/gql", post(errors::ping))
        .route("/ping", get(errors::ping))
        .route("/ws", get(errors::ping))
        .fallback(errors::not_found)
}
