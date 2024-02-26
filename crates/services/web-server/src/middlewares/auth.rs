use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

use crate::utils::response::Result;
use crate::utils::response::status_code::StatusCode;

pub const AUTH_COOKIE_KEY: &str = "auth";

pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request,
    next: Next,
) -> Result<Response> {
    println!("--> MIDDLEWARE - mw_require_auth");
    println!("--> MIDDLEWARE - cookies: {:?}", cookies);
    // 检查是否存在名为 "auth" 的 cookie
    // 如果存在，则继续处理请求，如果不存在，则返回 401 Unauthorized
    // 如果需要，这里还可以进行一些权限验证等操作

    let result = cookies.get(AUTH_COOKIE_KEY).map(|c| c.value().to_string());

    if result.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}