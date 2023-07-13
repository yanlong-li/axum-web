use axum::response::{IntoResponse, Response};

use crate::utils::response::error;

pub type Result<T> = core::result::Result<T, Error>;


pub mod client;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    // -- Auth errors.
    AuthFailNotAuthTokenCookie,
    AuthFailTokenWrongFormat,

    // -- More errors.
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("--> {:<12} - {self:?}", "INTO_RES");

        let client_error = self.client_status_and_error();

        error(client_error.0, client_error.1).into_response()
    }
}

impl Error {
    fn client_status_and_error(&self) -> client::StatusCode {
        match self {
            Self::AuthFailTokenWrongFormat |
            Self::AuthFailNotAuthTokenCookie => client::StatusCode::NO_AUTH,
            _ => client::StatusCode::SERVER_ERROR
        }
    }
}
