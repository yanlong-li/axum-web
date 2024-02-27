use axum::{Extension, Json, response::Result};
use axum::extract::Path;
use axum::response::Response;
use redis::{AsyncCommands, Client as RedisClient, RedisResult};
use serde::{Deserialize, Serialize};
use serde_json;

use lib_core::model::store::DbPool;
use lib_core::model::user::{create, find_by_username, find_all, User};

use crate::utils::response::{error, success};
use crate::utils::response::status_code::StatusCode;

#[derive(Deserialize, Serialize)]
pub struct SearchUserByUsername {
    pub username: String,
}


pub async fn action_find_user(
    Path(path): Path<SearchUserByUsername>,
    Extension(pool): Extension<DbPool>,
    Extension(redis_client): Extension<RedisClient>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let mut redis_conn = redis_client.get_async_connection().await.unwrap();

    let cache: RedisResult<String> = redis_conn.get(&path.username).await;

    match cache {
        Ok(json) => {
            let user: User = serde_json::from_str::<User>(&json).unwrap();
            Ok(Json(user))
        }
        Err(_) => {
            let user_result = find_by_username(&pool, &path.username).await;

            match user_result {
                Ok(user) => {
                    let json = serde_json::to_string(&user).unwrap();
                    redis_conn.set_ex::<&str, String, ()>(&path.username, json, 5).await.unwrap();
                    Ok(Json(user))
                }
                Err(_) => Err((axum::http::StatusCode::NOT_FOUND, "Not Found".to_string()))
            }
        }
    }
}

#[axum_macros::debug_handler]
pub async fn action_create_user(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<CreateUser>,
) -> Response {
    if payload.username.trim().is_empty() {
        return error(StatusCode::USERNAME_CANNOT_BE_EMPTY);
    }

    let user = create(&pool, &payload.username).await;

    match user {
        Ok(user) => {
            success(Some(user))
        }
        Err(err) => {
            tracing::warn!("{}", format!("{}",err.to_string()));
            error(StatusCode::USERNAME_OR_PASSWORD_MISMATCH)
        }
    }
}

// the input to our `create_user` handler
#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    username: String,
}

pub async fn action_list(
    Extension(pool): Extension<DbPool>,
) -> Json<Vec<User>> {
    let users_result = find_all(&pool).await.unwrap();
    Json(users_result)
}

