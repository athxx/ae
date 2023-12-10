use axum::{
    http::{
        header::{HeaderValue, CONTENT_TYPE},
        HeaderMap,
    },
    response::IntoResponse,
};

pub fn resp_html(s: String) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    (headers, s)
}
