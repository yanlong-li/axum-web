use axum::{Extension, Json, response::Result};
use axum::extract::Path;
use axum::http::StatusCode;
use redis::{AsyncCommands, Client as RedisClient, RedisResult};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::mysql::MySqlPool;

use crate::models::users::SearchUserByUsername;
use crate::schema::user::User;

pub async fn action_find_user(
    Path(path): Path<SearchUserByUsername>,
    Extension(pool): Extension<MySqlPool>,
    Extension(redis_client): Extension<RedisClient>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut redis_conn = redis_client.get_async_connection().await.unwrap();

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
                    redis_conn.set_ex::<&str, String, ()>(&path.username, json, 5).await.unwrap();
                    Ok(Json(user))
                }
                Err(_) => Err((StatusCode::NOT_FOUND, "Not Found".to_string()))
            }
        }
    }
}

// 假设你有一个创建用户的函数
pub async fn action_create_user(
    // 这个参数类型实现了FromRequestParts trait
    Json(payload): Json<CreateUser>,
    Extension(pool): Extension<MySqlPool>,
) -> (StatusCode, Json<User>) {
    // 插入用户数据并返回id
    let user = crate::schema::user::create(payload.username).await;

    // 这个返回类型实现了IntoResponse trait
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct CreateUser {
    username: String,
}

pub async fn action_list(
    Extension(pool): Extension<MySqlPool>,
) -> Json<Vec<User>> {
    let users_result = sqlx::query_as::<_, User>("SELECT id,username FROM user")
        .fetch_all(&pool)
        .await.unwrap();
    Json(users_result)
}

