#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClientStatusCode(pub u32, pub &'static str);

#[allow(unused_macros)]
macro_rules! client_status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl ClientStatusCode {
        $(
            $(#[$docs])*
            pub const $konst: ClientStatusCode = ClientStatusCode($num,$phrase);
        )+

        }
    }
}

client_status_codes!(
    (0,SERVER_ERROR,"Server Error");
    (1,OK,"Ok");
    (2,UNAUTHORIZED,"Unauthorized");
    (3,USERNAME_CANNOT_BE_EMPTY,"User name cannot be empty");
    (4,INCORRECT_USERNAME_FORMAT,"Incorrect username format");
    (5,USERNAME_OR_PASSWORD_MISMATCH,"User name or password mismatch");
    (6,NOT_FOUND_IP,"NOT_FOUND_IP");
);