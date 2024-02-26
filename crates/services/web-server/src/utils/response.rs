use axum::Json;
use axum::response::{self, IntoResponse};
use serde::{Deserialize, Serialize};
use crate::utils::response::status_code::StatusCode;

pub mod status_code;

pub type Result<T> = core::result::Result<T, StatusCode>;

#[derive(Serialize, Deserialize)]
pub struct Response<T>
{
    pub code: u32,
    pub msg: &'static str,
    pub data: Option<T>,
}

impl Default for Response<()> {
    fn default() -> Self {
        Response {
            code: StatusCode::OK.0,
            msg: StatusCode::OK.1,
            data: None,
        }
    }
}

pub fn success_without_data() -> response::Response {
    Json(Response::default()).into_response()
}

pub fn success<T>(data: Option<T>) -> response::Response
    where
        T: Serialize,
{
    let client_status = StatusCode::OK;

    Json(Response {
        code: client_status.0,
        msg: client_status.1,
        data,
    }).into_response()
}

pub fn error(status: StatusCode) -> response::Response
{
    Json(Response {
        code: status.0,
        msg: status.1,
        data: None::<String>,
    }).into_response()
}