use axum::Json;
use axum::response;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

pub use error::Error;
pub use error::Result;

use crate::utils::response::error::client;

mod error;

#[derive(Serialize, Deserialize)]
pub struct ResponseContent<T> {
    pub code: u32,
    pub msg: &'static str,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub code: u32,
    pub msg: &'static str,
}


pub fn success<T>(data: T) -> response::Response
    where
        T: Serialize,
{
    let client_status = client::StatusCode::OK;

    Json(ResponseContent {
        code: client_status.0,
        msg: client_status.1,
        data,
    }).into_response()
}

pub fn error(status: client::StatusCode) -> response::Response
{
    Json(Response {
        code: status.0,
        msg: status.1,
    }).into_response()
}