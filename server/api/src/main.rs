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
pub struct Arg {
    pub port_api: String,
    pub port_adm: String,
}

#[tokio::main]
async fn main() {
    let vars = Arg {
        port_api: String::from("0.0.0.0:3000"),
        port_adm: String::from("0.0.0.0:3001"),
    };
    // initialize tracing
    tracing_subscriber::fmt::init();
    //fast_log::init(fast_log::Config::new().console().file("test.log").chan_len(Some(100000))).unwrap();
    // run our app with hyper
    info!("server running on : {}", vars.port_api);
    axum::Server::bind(&vars.port_api.parse().unwrap())
        .serve(handler::router().await.into_make_service())
        .await
        .unwrap();
}
