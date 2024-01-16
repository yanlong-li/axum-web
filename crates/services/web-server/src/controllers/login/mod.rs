use axum::{Extension, Json};
use axum::response::Response;
use lib_core::model::store::DbPool;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;
use lib_core::model::user::find_by_username;

use crate::models::{UserInfo, UserLogin, UserLoginSuccess};
use crate::utils::response::{error, success};
use crate::utils::response::client::ClientStatusCode;

pub async fn action_login(
    cookies: Cookies,
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<UserLogin>,
) -> Response {
    if payload.username.trim().is_empty() {
        error(ClientStatusCode::USERNAME_CANNOT_BE_EMPTY)
    } else if payload.username.trim().chars().all(char::is_alphabetic) {
        error(ClientStatusCode::INCORRECT_USERNAME_FORMAT)
    } else {
        let user_result = find_by_username(&pool, &payload.username)
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