//! Virtual key codes used in keyboard events.
//!
//! | Codes                       | Dependent on keyboard layout |
//! | :-------------------------- | :--------------------------- |
//! | `ANSI_` prefix              | **yes** |
//! | `RETURN` through `UP_ARROW` | **no**  |

#![allow(missing_docs)]

macro_rules! keys {
    ($($name:ident = $val:expr,)+) => {
        $(
            pub const $name: u16 = $val;
        )+
    };
}

keys! {
    // Can map to different keys depending on keyboard layout.
    ANSI_0                = 0x1D,
    ANSI_1                = 0x12,
    ANSI_2                = 0x13,
    ANSI_3                = 0x14,
    ANSI_4                = 0x15,
    ANSI_5                = 0x17,
    ANSI_6                = 0x16,
    ANSI_7                = 0x1A,
    ANSI_8                = 0x1C,
    ANSI_9                = 0x19,
    ANSI_ALPHA_A          = 0x00,
    ANSI_ALPHA_B          = 0x0B,
    ANSI_ALPHA_C          = 0x08,
    ANSI_ALPHA_D          = 0x02,
    ANSI_ALPHA_E          = 0x0E,
    ANSI_ALPHA_F          = 0x03,
    ANSI_ALPHA_G          = 0x05,
    ANSI_ALPHA_H          = 0x04,
    ANSI_ALPHA_I          = 0x22,
    ANSI_ALPHA_J          = 0x26,
    ANSI_ALPHA_K          = 0x28,
    ANSI_ALPHA_L          = 0x25,
    ANSI_ALPHA_M          = 0x2E,
    ANSI_ALPHA_N          = 0x2D,
    ANSI_ALPHA_O          = 0x1F,
    ANSI_ALPHA_P          = 0x23,
    ANSI_ALPHA_Q          = 0x0C,
    ANSI_ALPHA_R          = 0x0F,
    ANSI_ALPHA_S          = 0x01,
    ANSI_ALPHA_T          = 0x11,
    ANSI_ALPHA_U          = 0x20,
    ANSI_ALPHA_V          = 0x09,
    ANSI_ALPHA_W          = 0x0D,
    ANSI_ALPHA_X          = 0x07,
    ANSI_ALPHA_Y          = 0x10,
    ANSI_ALPHA_Z          = 0x06,
    ANSI_BACKSLASH        = 0x2A,
    ANSI_COMMA            = 0x2B,
    ANSI_EQUAL            = 0x18,
    ANSI_GRAVE            = 0x32,
    ANSI_KEYPAD_0         = 0x52,
    ANSI_KEYPAD_1         = 0x53,
    ANSI_KEYPAD_2         = 0x54,
    ANSI_KEYPAD_3         = 0x55,
    ANSI_KEYPAD_4         = 0x56,
    ANSI_KEYPAD_5         = 0x57,
    ANSI_KEYPAD_6         = 0x58,
    ANSI_KEYPAD_7         = 0x59,
    ANSI_KEYPAD_8         = 0x5B,
    ANSI_KEYPAD_9         = 0x5C,
    ANSI_KEYPAD_CLEAR     = 0x47,
    ANSI_KEYPAD_DECIMAL   = 0x41,
    ANSI_KEYPAD_DIVIDE    = 0x4B,
    ANSI_KEYPAD_ENTER     = 0x4C,
    ANSI_KEYPAD_EQUALS    = 0x51,
    ANSI_KEYPAD_MINUS     = 0x4E,
    ANSI_KEYPAD_MULTIPLY  = 0x43,
    ANSI_KEYPAD_PLUS      = 0x45,
    ANSI_LEFT_BRACKET     = 0x21,
    ANSI_MINUS            = 0x1B,
    ANSI_PERIOD           = 0x2F,
    ANSI_QUOTE            = 0x27,
    ANSI_RIGHT_BRACKET    = 0x1E,
    ANSI_SEMICOLON        = 0x29,
    ANSI_SLASH            = 0x2C,
    // Independent of keyboard layout
    CAPS_LOCK       = 0x39,
    COMMAND         = 0x37,
    CONTROL         = 0x3B,
    DELETE          = 0x33,
    DOWN_ARROW      = 0x7D,
    END             = 0x77,
    ESCAPE          = 0x35,
    F01             = 0x7A,
    F02             = 0x78,
    F03             = 0x63,
    F04             = 0x76,
    F05             = 0x60,
    F06             = 0x61,
    F07             = 0x62,
    F08             = 0x64,
    F09             = 0x65,
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
    FORWARD_DELETE  = 0x75,
    FUNCTION        = 0x3F,
    HELP            = 0x72,
    HOME            = 0x73,
    LEFT_ARROW      = 0x7B,
    MUTE            = 0x4A,
    OPTION          = 0x3A,
    PAGE_DOWN       = 0x79,
    PAGE_UP         = 0x74,
    RETURN          = 0x24,
    RIGHT_ARROW     = 0x7C,
    RIGHT_COMMAND   = 0x36,
    RIGHT_CONTROL   = 0x3E,
    RIGHT_OPTION    = 0x3D,
    RIGHT_SHIFT     = 0x3C,
    SHIFT           = 0x38,
    SPACE           = 0x31,
    TAB             = 0x30,
    UP_ARROW        = 0x7E,
    VOLUME_DOWN     = 0x49,
    VOLUME_UP       = 0x48,
}
