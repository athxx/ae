#![allow(dead_code)]
#![allow(unused)]

mod handler;

use axum::{
    http::{header, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use log::info;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let cfg = core::common::cfg::get_or_init();

    // initialize tracing
    tracing_subscriber::fmt::init();
    //fast_log::init(fast_log::Config::new().console().file("test.log").chan_len(Some(100000))).unwrap();
    // run our app with hyper
    info!("server running on : {}", cfg.port_api);
    axum::Server::bind(&cfg.port_api.parse().unwrap())
        .serve(handler::router().await.into_make_service())
        .await
        .unwrap();
}
