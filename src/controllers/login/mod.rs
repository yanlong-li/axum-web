use axum::Json;
use axum::response::Response;
use uuid::Uuid;

use crate::databases::get_db;
use crate::models::{UserInfo, UserLogin, UserLoginSuccess};
use crate::schemas::User;
use crate::utils::response::{error, success};

pub async fn action_login(
    Json(payload): Json<UserLogin>
) -> Response {
    if payload.username.trim().is_empty() {
        error(1, "用户名不能为空")
    } else {
        let conn = get_db().await;

        let user_result = sqlx::query_as::<_, User>("SELECT id,username FROM `user` WHERE username = ?")
            .bind(&payload.username)
            .fetch_one(&conn)
            .await;

        match user_result {
            Ok(user) => {
                let token = Uuid::new_v4();

                success(UserLoginSuccess {
                    token: token.to_string(),
                    user: UserInfo {
                        id: user.id,
                        username: user.username,
                    },
                })
            }
            Err(_) => error(1, "账号不存在")
        }
        //
        //
        // success(User {
        //     id: 1,
        //     username: "2".to_string(),
        // })
    }
}