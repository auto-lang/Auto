//! 🎨 Color utilities.

/// A simple RGB color.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rgb {
    /// The amount of red, ranging from 0.0 to 1.0.
    pub red: u8,
    /// The amount of green, ranging from 0.0 to 1.0.
    pub green: u8,
    /// The amount of green, ranging from 0.0 to 1.0.
    pub blue: u8,
}
