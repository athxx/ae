use crate::db::PrismaClient;
use tokio::sync::OnceCell;

static DB: OnceCell<PrismaClient> = OnceCell::const_new();

pub async fn get_or_init() -> &'static PrismaClient {
    DB.get_or_init(|| async { PrismaClient::_builder().build().await.unwrap() }).await
}
