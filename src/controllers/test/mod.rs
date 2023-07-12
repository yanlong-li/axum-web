use axum::headers::Cookie;
use axum::TypedHeader;

pub async fn action_cookie(TypedHeader(cookie): TypedHeader<Cookie>) -> String {
    cookie.get("language").unwrap_or_else(|| {
        "None"
    }).to_string()
}