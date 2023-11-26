mod errors;
mod index;

use axum::{
    http::{header, HeaderValue, Method, StatusCode},
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use tower_http::cors::CorsLayer;

pub async fn router() -> Router {
    Router::new()
        // public
        .route("/", get(index::index))
        // system
        .route("/robots.txt", get(index::index))
        .route("/sitemap.xml", get(index::index))
        // non-permissions
        .route("/sign/in", get(index::index))
        .route("/sign/up", get(index::index))
        .route("/sign/out", get(index::index))
        .route("/captcha/sms", get(errors::ping))
        .route("/captcha/email", get(errors::ping))
        // test
        .route("/c", post(index::index))
        // websocket & webtransport
        .route("/ws/jwtToken", get(errors::ping))
        .route("/ws/:id", get(errors::ping))
        // USER
        .route("/user/info", get(errors::ping))
        .route("/user/edit", get(errors::ping))
        // USERS
        .route("/users/list", get(errors::ping))
        // GEO
        .route("/users/geo", get(errors::ping))
        // SHOP
        .route("/shop", get(errors::ping))
        // jobs
        .route("/job/detail", get(errors::ping))
        // search
        .route("/search", get(errors::ping))
        // meetup
        .route("/meetup", get(errors::ping))
        // news
        .route("/news", get(errors::ping))
        // im
        .route("/im", get(errors::ping)).fallback(errors::not_found)
}

// pub async fn routes() -> Router {
//     Router::new().nest(path, router)
// }
