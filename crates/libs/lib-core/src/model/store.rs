use sqlx::{Executor, MySqlPool};


pub type DbPool = MySqlPool;

pub async fn get_db(url: &str) -> DbPool {
    let pool = MySqlPool::connect(url)
        .await
        .unwrap();

    // set timezone Asia/shanghai
    let _ = pool.execute("set time_zone = 'Asia/Shanghai'").await;

    return pool;
}