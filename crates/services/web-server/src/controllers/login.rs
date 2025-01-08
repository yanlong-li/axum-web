use axum::response::Response;
use axum::{Extension, Json};
use axum_session::{Session};
use axum_session_redispool::SessionRedisPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use lib_core::model::store::DbPool;
use lib_core::model::user::find_by_username;

use crate::utils::response::status_code::StatusCode;
use crate::utils::response::{error, success};

#[derive(Deserialize, Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserLoginSuccess {
    pub user: UserInfo,
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
}


pub async fn action_login(
    Extension(pool): Extension<DbPool>,
    session: Session<SessionRedisPool>,
    Json(payload): Json<UserLogin>,
) -> Response {
    println!("{:?}", session);

    if payload.username.is_empty() {
        error(StatusCode::USERNAME_CANNOT_BE_EMPTY)
    } else {
        let user_result = find_by_username(&pool, &payload.username)
            .await;

        match user_result {
            Ok(user) => {
                let token = Uuid::new_v4();
                session.set("user_id", user.id);
                success(Some(UserLoginSuccess {
                    token: token.to_string(),
                    user: UserInfo {
                        id: user.id,
                        username: user.username,
                    },
                }))
            }
            Err(err) => {
                tracing::warn!("{}",err.to_string());
                error(StatusCode::USERNAME_OR_PASSWORD_MISMATCH)
            }
        }
    }
}