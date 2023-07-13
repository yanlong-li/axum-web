#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(pub u32,pub &'static str);

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

        // fn canonical_reason(num: u32) -> Option<&'static str> {
        //     match num {
        //         $(
        //         $num => Some($phrase),
        //         )+
        //         _ => None
        //     }
        // }
    }
}


status_codes!(
    (0,SERVER_ERROR,"Server Error");
    (1,OK,"Ok");
    (2,NO_AUTH,"No Auth");
);