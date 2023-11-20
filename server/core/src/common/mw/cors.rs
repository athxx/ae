use axum::{
    http::{header, HeaderMap, Method, Request, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

pub async fn cors<B>(req: Request<B>, next: Next<B>) -> Response {
    let origin = req.headers().get("origin");
    match origin {
        Some(val) => {
            // let header_mut = req.headers_mut();
            // header_mut.insert("Access-Control-Allow-Origin", val.to_owned());
            // header_mut.insert("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT, DELETE, UPDATE".parse().unwrap());
            // header_mut.insert(
            //     "Access-Control-Allow-Headers",
            //     "Origin, X-Requested-With, Content-Type, Accept, Authorization".parse().unwrap(),
            // );
            // header_mut.insert(
            //     "Access-Control-Expose-Headers",
            //     "Content-Length, Access-Control-Allow-Origin, Access-Control-Allow-Headers, Cache-Control, Content-Language, Content-Type"
            //         .parse()
            //         .unwrap(),
            // );
            // header_mut.insert("Access-Control-Allow-Credentials", "true".parse().unwrap());
            // header_mut.insert("Access-Control-Max-Age", "86400".parse().unwrap());

            // if req.method().as_str() == "OPTIONS" {
            //     let rsp = (StatusCode::INTERNAL_SERVER_ERROR, [("x-foo", "bar")], "Something went wrong...").into_response();
            //     rsp
            // }

            next.run(req).await
        }
        None => next.run(req).await,
    }
}
