use aws_config;
use aws_sdk_s3;

pub async fn init() {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);
    client.create_multipart_upload();
}
