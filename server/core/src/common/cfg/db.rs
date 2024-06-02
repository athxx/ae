use tokio::sync::OnceCell;

static DB: OnceCell<Vec<int>> = OnceCell::const_new();

pub async fn get_db() -> &'static Vec<int> {
    DB.get_or_init(|| async { vec![123] }).await
}
