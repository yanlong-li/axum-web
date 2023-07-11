use serde::{Deserialize, Serialize};

// the output to our `create_user` handler
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: u64,
    pub username: String,
}


