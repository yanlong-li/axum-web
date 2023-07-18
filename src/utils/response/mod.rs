use axum::Json;
use axum::response::{self, IntoResponse};
use serde::{Deserialize, Serialize};

pub mod server;
pub mod client;

pub type Result<T> = core::result::Result<T, server::ServerStatusCode>;

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
    let client_status = client::ClientStatusCode::OK;

    Json(ResponseContent {
        code: client_status.0,
        msg: client_status.1,
        data,
    }).into_response()
}

pub fn error(status: client::ClientStatusCode) -> response::Response
{
    Json(Response {
        code: status.0,
        msg: status.1,
    }).into_response()
}