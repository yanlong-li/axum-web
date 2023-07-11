use axum::response::Result;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

// the output to our `create_user` handler
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: u64,
    pub username: String,
}


pub async fn create(username: String) -> Result<User, (StatusCode, String)> {

    Ok(User{
        id: 0,
        username
    })
}