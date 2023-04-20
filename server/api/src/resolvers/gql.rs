use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    http::header::HeaderMap,
    response::{Html, IntoResponse},
    Router,
};
use core::service::schema::AppSchema;

// graphql playground
pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/api").subscription_endpoint("/ws").finish())
}

pub async fn graphql_handler(schema: Extension<AppSchema>, headers: HeaderMap, req: GraphQLRequest) -> GraphQLResponse {
    // let mut req = req.into_inner();
    // if let Some(token) = get_token_from_headers(&headers) {
    //     req = req.data(token);
    // }
    schema.execute(req.into_inner()).await.into()
}

// TODO get auth token from headers, it should be a middleware
// fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
//     headers.get("Token").and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
// }
