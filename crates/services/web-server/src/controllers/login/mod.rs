use axum::{Extension, Json};
use axum::response::Response;
use sqlx::MySqlPool;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::active_records::User;
use crate::models::{UserInfo, UserLogin, UserLoginSuccess};
use crate::utils::response::{error, success};
use crate::utils::response::client::ClientStatusCode;

pub async fn action_login(
    cookies: Cookies,
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<UserLogin>,
) -> Response {
    if payload.username.trim().is_empty() {
        error(ClientStatusCode::USERNAME_CANNOT_BE_EMPTY)
    } else if payload.username.trim().chars().all(char::is_alphabetic) {
        error(ClientStatusCode::INCORRECT_USERNAME_FORMAT)
    } else {

        let user_result = sqlx::query_as::<_, User>("SELECT id,username FROM `user` WHERE username = ?")
            .bind(&payload.username)
            .fetch_one(&pool)
            .await;

        match user_result {
            Ok(user) => {
                let token = Uuid::new_v4();
                cookies.add(Cookie::new("auth", token.to_string()));

                success(Some(UserLoginSuccess {
                    token: token.to_string(),
                    user: UserInfo {
                        id: user.id,
                        username: user.username,
                    },
                }))
            }
            Err(_) => error(ClientStatusCode::USERNAME_OR_PASSWORD_MISMATCH)
        }
    }
}