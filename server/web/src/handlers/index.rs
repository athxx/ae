use super::view;
use axum::{
    http::{
        header::{HeaderValue, CONTENT_TYPE},
        HeaderMap,
    },
    response::IntoResponse,
};

pub fn render(s: String) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    (headers, s)
}

pub async fn index() -> impl IntoResponse {
    render(view::index("23343434".to_owned(), vec![String::from("foo"), String::from("bar")]))
}

pub async fn topic() -> &'static str {
    "pong"
}

pub async fn post() -> &'static str {
    "dd"
}
