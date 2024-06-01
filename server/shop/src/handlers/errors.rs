use axum::{
    http::{header, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

/// ping pong
pub async fn ping() -> &'static str {
    "pong"
}

/// 204 no content
pub async fn no_content() -> impl IntoResponse {
    (StatusCode::NO_CONTENT)
}

/// 404 not found
pub async fn not_found(method: Method) -> impl IntoResponse {
    if method == "GET" {
        (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/html")],
            r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <title>404 Not Found</title>
    <style>
      body {
        font-family: Arial, sans-serif;
        background-color: #f5f5f5;
        color: #444;
        text-align: center;
        padding-top: 50px;
      }
      h1 {
        font-size: 36px;
        margin-bottom: 20px;
      }
      p {
        font-size: 18px;
        margin-bottom: 20px;
      }
      a {
        color: #0078e7;
        text-decoration: none;
      }
      a:hover {
        text-decoration: underline;
      }
    </style>
  </head>
  <body>
    <h1>404 Not Found</h1>
    <p>Sorry, the page you're looking for could not be found.</p>
    <p>
      Please check the URL in the address bar, or
      <a href="/">go to the homepage</a>.
    </p>
  </body>
</html>"#,
        )
    } else {
        (StatusCode::NOT_FOUND, [(header::CONTENT_TYPE, "application/json")], r#"{"code":404,"message":"Not Found"}"#)
    }
}
