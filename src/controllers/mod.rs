pub mod ws;
pub mod users;
pub mod status;
pub mod test;
pub mod login;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}