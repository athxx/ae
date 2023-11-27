pub mod robots;
pub mod rss;
pub mod sitemap;



pub async fn robots() -> impl IntoResponse {

    resp_html(view::index("23343434".to_owned(), vec![String::from("foo"), String::from("bar")]))
}

pub async fn rss() -> impl IntoResponse {

    resp_html(view::index("23343434".to_owned(), vec![String::from("foo"), String::from("bar")]))
}

pub async fn sitemap() -> impl IntoResponse {

    resp_html(view::index("23343434".to_owned(), vec![String::from("foo"), String::from("bar")]))
}