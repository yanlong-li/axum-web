use serde::{Deserialize, Serialize};

// 定义一个结构体来表示查询参数
#[derive(Deserialize, Serialize)]
pub struct SearchUserByUsername {
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserLoginSuccess {
    pub user: UserInfo,
    pub token: String,
}