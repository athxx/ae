use tokio::sync::OnceCell;

static DB: OnceCell<Vec<i32>> = OnceCell::const_new();

pub async fn get_db() -> &'static Vec<i32> {
    DB.get_or_init(|| async { vec![123] }).await
}
