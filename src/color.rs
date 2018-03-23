//! 🎨 Color utilities.

use std::mem;

macro_rules! impl_convert {
    ($t:ty; $size:expr) => {
        impl From<[u8; $size]> for $t {
            #[inline]
            fn from(arr: [u8; $size]) -> Self {
                unsafe { mem::transmute(arr) }
            }
        }

        impl From<$t> for [u8; $size] {
            #[inline]
            fn from(color: $t) -> Self {
                unsafe { mem::transmute(color) }
            }
        }

        impl AsRef<[u8]> for $t {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                let ptr = self as *const Self as *const [u8; $size];
                unsafe { &*ptr }
            }
        }

        impl AsMut<[u8]> for $t {
            #[inline]
            fn as_mut(&mut self) -> &mut [u8] {
                let ptr = self as *mut Self as *mut [u8; $size];
                unsafe { &mut *ptr }
            }
        }
    };
    ($t:ty) => {
        impl_convert! { $t; mem::size_of::<$t>() }
    };
}

/// A simple RGB color.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Rgb {
    /// The amount of red, ranging from 0 to 255.
    pub red: u8,
    /// The amount of green, ranging from 0 to 255.
    pub green: u8,
    /// The amount of green, ranging from 0 to 255.
    pub blue: u8,
}

impl_convert! { Rgb }
