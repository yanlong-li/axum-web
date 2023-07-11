use serde::{Deserialize, Serialize};

// 定义一个结构体来表示查询参数
#[derive(Deserialize, Serialize)]
pub struct SearchUserByUsername {
    pub username: String,
}