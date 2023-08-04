use crate::db;
pub async fn get_list() {
    let dto = db::PrismaClient::_builder().build().await.unwrap();
}
