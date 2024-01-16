use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

use crate::utils::response::Result;

pub const AUTH_COOKIE_KEY: &str = "auth";

pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request,
    next: Next,
) -> Result<Response> {
    println!("--> {:<12} - mw_require_auth ", "MIDDLEWARE");

    let _ = cookies.get(AUTH_COOKIE_KEY).map(|c| c.value().to_string());

    Ok(next.run(req).await)
}