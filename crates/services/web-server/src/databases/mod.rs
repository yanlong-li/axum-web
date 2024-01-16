use sqlx::MySqlPool;

pub async fn get_db(url: &str) -> MySqlPool {
    MySqlPool::connect(url)
        .await
        .unwrap()
}