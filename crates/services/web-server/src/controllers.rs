pub mod ws;
pub mod users;
pub mod status;
pub mod cookie;
pub mod login;
pub mod ip;
pub mod http;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}