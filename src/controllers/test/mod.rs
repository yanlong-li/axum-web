use tower_cookies::{Cookie, Cookies};

pub async fn action_cookie(cookies: Cookies) -> String {
    let cookie = cookies.get("language").unwrap_or(Cookie::new("language", "zh-CN"));

    cookies.add(Cookie::new("language", "zh-CN"));

    cookie.value().to_string()
}