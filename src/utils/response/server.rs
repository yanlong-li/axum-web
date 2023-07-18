use axum::response::{IntoResponse, Response};

use crate::utils::response::error;

use super::client::ClientStatusCode;


#[derive(Debug)]
#[allow(unused)]
pub enum ServerStatusCode {
    // -- Login errors.
    LoginFail,
    UsernameCannotBeEmpty,
    UsernameOrPasswordMismatch,

    // -- Auth errors.
    AuthFailNotAuthTokenCookie,
    AuthFailTokenWrongFormat,

    // -- More errors.
}

impl IntoResponse for ServerStatusCode {
    fn into_response(self) -> Response {
        println!("--> {:<12} - {self:?}", "INTO_RES");

        error(self.client_status_and_error()).into_response()
    }
}

impl ServerStatusCode {
    fn client_status_and_error(&self) -> ClientStatusCode {
        match self {
            Self::UsernameCannotBeEmpty => ClientStatusCode::USERNAME_CANNOT_BE_EMPTY,
            Self::UsernameOrPasswordMismatch => ClientStatusCode::USERNAME_OR_PASSWORD_MISMATCH,
            Self::AuthFailTokenWrongFormat |
            Self::AuthFailNotAuthTokenCookie => ClientStatusCode::UNAUTHORIZED,
            _ => ClientStatusCode::SERVER_ERROR
        }
    }
}
