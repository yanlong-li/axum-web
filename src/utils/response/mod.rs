use axum::Json;
use axum::response;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseContent<T> {
    pub code: u32,
    pub msg: String,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub code: u32,
    pub msg: String,
}


pub fn success<T>(data: T) -> response::Response
    where
        T: Serialize,
{
    Json(ResponseContent {
        code: 200,
        msg: "SUCCESS".to_string(),
        data,
    }).into_response()
}

pub fn error(code: u32, msg: &str) -> response::Response
{
    Json(Response {
        code,
        msg: msg.to_string(),
    }).into_response()
}