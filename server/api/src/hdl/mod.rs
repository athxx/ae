mod auth;
mod errors;
mod user;

use axum::{
    http::{header, Method, StatusCode},
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

pub async fn router() -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/ping", get(errors::ping))
                // public
                .route("/public/home", post(user::set::create_user))
                .route("/public/promote", post(user::set::create_user))
                // non-permissions
                .route("/login", get(auth::login))
                .route("/register", get(auth::register))
                .route("/captcha/sms", get(errors::ping))
                .route("/captcha/email", get(errors::ping))
                // test
                .route("/c", post(user::set::create_user))
                // websocket
                .route("/ws/jwtToken", get(errors::ping))
                // user
                .route("/user/info", get(errors::ping))
                // geo
                .route("/geo/users", get(errors::ping))
                // shop
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
                .route("/im", get(errors::ping)),
        )
        .fallback(errors::not_found)
}

// pub async fn routes() -> Router {
//     Router::new().nest(path, router)
// }
