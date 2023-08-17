mod auth;
mod errors;
mod user;

use axum::{
    http::{header, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

pub async fn router() -> Router {
    Router::new()
        .route("/", get(errors::ping))
        .route("/ping", get(errors::ping))
        // non-permissions
        .route("/api/login", get(auth::login))
        .route("/api/register", get(auth::register))
        .route("/api/ws", get(errors::ping))
        // test
        .route("/c", post(user::set::create_user))
        .fallback(errors::not_found)
}

// pub async fn routes() -> Router {
//     Router::new().nest(path, router)
// }
