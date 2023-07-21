use axum::Json;
use axum::response::{self, IntoResponse};
use serde::{Deserialize, Serialize};

pub mod server;
pub mod client;

pub type Result<T> = core::result::Result<T, server::ServerStatusCode>;

#[derive(Serialize, Deserialize)]
pub struct Response<T>
{
    pub code: u32,
    pub msg: &'static str,
    pub data: Option<T>,
}

impl<T> Default for Response<T> {
    fn default() -> Self {
        Response {
            code: client::ClientStatusCode::OK.0,
            msg: client::ClientStatusCode::OK.1,
            data: None::<T>,
        }
    }
}


pub fn success<T>(data: Option<T>) -> response::Response
    where
        T: Serialize,
{
    let client_status = client::ClientStatusCode::OK;

    Json(Response {
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
        data: None::<String>,
    }).into_response()
}