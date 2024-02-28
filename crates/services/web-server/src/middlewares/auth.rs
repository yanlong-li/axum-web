use axum::Extension;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum_session::{ReadOnlySession, SessionRedisPool};
use serde::{Deserialize, Serialize};
use lib_core::model::store::DbPool;

use crate::utils::response::Result;
use crate::utils::response::status_code::StatusCode;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
}

// 这里一定要注意，req、next 要在最后
pub async fn mw_require_auth(
    session: ReadOnlySession<SessionRedisPool>,
    Extension(pool): Extension<DbPool>,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    tracing::debug!("--> MIDDLEWARE - mw_require_auth");

    // 需要获取 session 并判断数据
    let user_result = session.get::<u64>("user_id");

    match user_result {
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
        Some(user_id) => {
            tracing::info!("--> MIDDLEWARE - user_id: {}", user_id);

            let user = lib_core::model::user::find_one(&pool, &user_id).await;

            match user {
                Ok(user) => {
                    req.extensions_mut().insert(user);
                }
                Err(_) => {
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
        }
    }

    Ok(next.run(req).await)
}