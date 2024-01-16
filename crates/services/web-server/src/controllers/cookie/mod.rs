use axum::response::Response;
use tower_cookies::{Cookie, Cookies};

use crate::utils::response::success;

pub async fn action_cookie_language(cookies: Cookies) -> Response {
    let cookie = cookies.get("language").unwrap_or(Cookie::new("language", "zh-CN"));

    cookies.add(Cookie::new("language", "zh-CN"));

    success(Some(cookie.value()))
}