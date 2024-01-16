use sqlx::MySqlPool;

pub async fn get_db() -> MySqlPool {
    MySqlPool::connect("mysql://root:123456@localhost/axum")
        .await
        .unwrap()
}