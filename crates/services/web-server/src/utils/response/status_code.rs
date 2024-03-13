#![allow(dead_code)]
use axum::response::{IntoResponse, Response};
use crate::utils::response::from_status_code;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(pub u32, pub &'static str);

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        from_status_code(self)
    }
}

#[allow(unused_macros)]
macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl StatusCode {
        $(
            $(#[$docs])*
            pub const $konst: StatusCode = StatusCode($num,$phrase);
        )+

        }
    }
}
status_codes!(
    (0,INTERNAL_SERVER_ERROR,"Server Error");
    (1,OK,"Ok");
    (2,UNAUTHORIZED,"Unauthorized");
    (3,USERNAME_CANNOT_BE_EMPTY,"User name cannot be empty");
    (4,INCORRECT_USERNAME_FORMAT,"Incorrect username format");
    (5,USERNAME_OR_PASSWORD_MISMATCH,"User name or password mismatch");
    (6,NOT_FOUND,"Not found");
);

