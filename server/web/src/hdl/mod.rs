mod errors;
mod index;

use axum::{
    http::{header, HeaderValue, Method, StatusCode},
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use log::info;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub fn router() -> Router {
    let assets = std::env::current_dir().unwrap();
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
        // static asset files
        .nest_service("/assets", ServeDir::new(format!("{}/templates/default/assets", assets.to_str().unwrap())))
}

// pub async fn routes() -> Router {
//     Router::new().nest(path, router)
// }
