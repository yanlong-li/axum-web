pub mod ws;
pub mod users;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}