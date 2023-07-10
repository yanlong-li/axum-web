use axum::{Extension, Json, response::Result};
use axum::extract::Path;
use axum::http::StatusCode;
use redis::{AsyncCommands, Client as RedisClient, RedisResult};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::mysql::MySqlPool;

// 定义一个结构体来表示查询参数
#[derive(Deserialize)]
pub struct UserPath {
    pub username: String,
}

pub async fn find_user(
    Path(path): Path<UserPath>,
    Extension(pool): Extension<MySqlPool>,
    Extension(redisClient): Extension<RedisClient>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut redis_conn = redisClient.get_async_connection().await.unwrap();

    let cache: RedisResult<String> = redis_conn.get(&path.username).await;

    match cache {
        Ok(json) => {
            let user: User = serde_json::from_str::<User>(&json).unwrap();
            Ok(Json(user))
        }
        Err(_) => {
            let user_result = sqlx::query_as::<_, User>("SELECT id,username FROM `user` WHERE username = ?")
                .bind(&path.username)
                .fetch_one(&pool)
                .await;


            match user_result {
                Ok(user) => {
                    let json = serde_json::to_string(&user).unwrap();
                    redis_conn.set_ex::<&str, String, usize>(&path.username, json, 5).await.unwrap();
                    Ok(Json(user))
                }
                Err(_) => Err((StatusCode::NOT_FOUND, "Not Found".to_string()))
            }
        }
    }
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    id: u64,
    username: String,
}