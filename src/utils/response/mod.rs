pub mod status;


use axum::response::ErrorResponse;
use crate::utils::response::status::StatusCode;

pub type Result<T = StatusCode, E = ErrorResponse> = axum::response::Result<T, E>;