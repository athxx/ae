use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error,
};

use std::fs::File;
use std::io::Read;

// HTTP GET method that returns the response body as a String
pub async fn get(url: &str, headers: Option<HeaderMap>) -> Result<String, Error> {
    let mut request = Client::new().client.get(url);
    if let Some(header_map) = headers {
        request = request.headers(header_map);
    }
    let response = request.send().await?;
    let body = response.text().await?;
    Ok(body)
}

// HTTP POST method that takes a JSON payload and returns the response body as a String
pub async fn post_json(url: &str, payload: &str, headers: Option<HeaderMap>) -> Result<String, Error> {
    let mut request = Client::new().client.post(url);
    if let Some(header_map) = headers {
        request = request.headers(header_map);
    }
    let response = request.header("Content-Type", "application/json").body(payload.to_owned()).send().await?;
    let body = response.text().await?;
    Ok(body)
}

pub async fn post_form(url: &str, data: HashMap<&str, &str>, headers: HeaderMap) -> Result<String, Error> {
    let client = Client::new();
    let response = client.post(url).headers(headers).form(&data).send()?;
    let body = response.text().await?;
    Ok(body)
}

pub async fn post_file(url: &str, file_path: &str) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;

    let client = Client::new();
    let response = client
        .post(url)
        .multipart(reqwest::multipart::Form::new().part("file", reqwest::multipart::Part::bytes(file_content).file_name(file_path)))
        .send()?;
    let body = response.text()?;
    Ok(body)
}

pub async fn delete(url: &str) -> Result<String, Error> {
    let client = Client::new();
    let response = client.delete(url).send().await?.text().await?;
    Ok(response)
}

pub fn put() {}

pub fn patch() {}

pub fn options() {}

pub fn head() {}
