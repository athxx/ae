use crate::db::PrismaClient;
use std::sync::OnceLock;

static DB: OnceLock<PrismaClient> = OnceLock::new();

pub async fn get_or_init() -> &'static PrismaClient {
    DB.get_or_init(|| -> PrismaClient {
        PrismaClient::_builder().build().await.unwrap();
    })
}
