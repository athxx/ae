#![allow(dead_code)]
#![allow(unused)]

mod hdl;
use core;

use axum::{
    http::{header, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};

use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_init();
    let cfg = core::common::cfg::get_cfg();
    //fast_log::init(fast_log::Config::new().console().file("test.log").chan_len(Some(100000))).unwrap();
    // run our app with hyper
    info!("LISTENING : {}", cfg.port_api);
    axum::Server::bind(&cfg.port_api.parse().unwrap())
        .serve(hdl::router().await.into_make_service())
        .await
        .unwrap();
}

async fn tracing_init() {
    // initialize tracing
    tracing_subscriber::fmt()
        // .without_time() // For early local development.
        // .with_target(false)
        // .with_ansi(true)
        .with_max_level(tracing::Level::TRACE) // TODO for development
        // .with_env_filter(EnvFilter::from_default_env())
        .init();

    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "axum_static_web_server=debug".into()))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
}