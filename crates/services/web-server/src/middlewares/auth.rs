use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum_session::{ReadOnlySession, SessionRedisPool};

use crate::utils::response::Result;
use crate::utils::response::status_code::StatusCode;


pub async fn mw_require_auth(
    session: ReadOnlySession<SessionRedisPool>,
    req: Request,
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
        }
    }

    Ok(next.run(req).await)
}