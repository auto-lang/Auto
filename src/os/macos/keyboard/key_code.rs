//! Virtual key codes used in keyboard events.
//!
//! Codes from `RETURN` through `UP_ARROW` are independent of keyboard layout.

macro_rules! keys {
    ($($name:ident = $val:expr),+ $(,)*) => {
        keys! { $(
            concat!("Key code for ", stringify!($name), ".");
            $name = $val,
        )+ }
    };
    ($($doc:expr; $name:ident = $val:expr,)+) => {
        $(
            #[doc = $doc]
            pub const $name: u16 = $val;
        )+
    };
}

keys! {
    // Independent of keyboard layout
    RETURN          = 0x24,
    TAB             = 0x30,
    SPACE           = 0x31,
    DELETE          = 0x33,
    ESCAPE          = 0x35,
    COMMAND         = 0x37,
    SHIFT           = 0x38,
    CAPS_LOCK       = 0x39,
    OPTION          = 0x3A,
    CONTROL         = 0x3B,
    RIGHT_COMMAND   = 0x36,
    RIGHT_SHIFT     = 0x3C,
    RIGHT_OPTION    = 0x3D,
    RIGHT_CONTROL   = 0x3E,
    FUNCTION        = 0x3F,
    VOLUME_UP       = 0x48,
    VOLUME_DOWN     = 0x49,
    MUTE            = 0x4A,
    F1              = 0x7A,
    F2              = 0x78,
    F3              = 0x63,
    F4              = 0x76,
    F5              = 0x60,
    F6              = 0x61,
    F7              = 0x62,
    F8              = 0x64,
    F9              = 0x65,
    F10             = 0x6D,
    F11             = 0x67,
    F12             = 0x6F,
    F13             = 0x69,
    F14             = 0x6B,
    F15             = 0x71,
    F16             = 0x6A,
    F17             = 0x40,
    F18             = 0x4F,
    F19             = 0x50,
    F20             = 0x5A,
    HELP            = 0x72,
    HOME            = 0x73,
    PAGE_UP         = 0x74,
    FORWARD_DELETE  = 0x75,
    END             = 0x77,
    PAGE_DOWN       = 0x79,
    LEFT_ARROW      = 0x7B,
    RIGHT_ARROW     = 0x7C,
    DOWN_ARROW      = 0x7D,
    UP_ARROW        = 0x7E,
}
