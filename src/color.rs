//! 🎨 Color utilities.

use std::hash;
use std::mem;
use std::slice;

macro_rules! impl_color {
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

        impl AsRef<[u8; $size]> for $t {
            #[inline]
            fn as_ref(&self) -> &[u8; $size] {
                let ptr = self as *const Self as *const [u8; $size];
                unsafe { &*ptr }
            }
        }

        impl AsMut<[u8; $size]> for $t {
            #[inline]
            fn as_mut(&mut self) -> &mut [u8; $size] {
                let ptr = self as *mut Self as *mut [u8; $size];
                unsafe { &mut *ptr }
            }
        }

        impl AsRef<[u8]> for $t {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                AsRef::<[u8; $size]>::as_ref(self)
            }
        }

        impl AsMut<[u8]> for $t {
            #[inline]
            fn as_mut(&mut self) -> &mut [u8] {
                AsMut::<[u8; $size]>::as_mut(self)
            }
        }

        // Clippy lint
        #[allow(derive_hash_xor_eq)]
        impl hash::Hash for $t {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                state.write(self.as_ref());
            }

            #[inline]
            fn hash_slice<H: hash::Hasher>(data: &[Self], state: &mut H) {
                let len = mem::size_of_val(data);
                let ptr = data.as_ptr() as *const u8;
                state.write(unsafe { slice::from_raw_parts(ptr, len) });
            }
        }
    };
    ($t:ty) => {
        impl_color! { $t; mem::size_of::<$t>() }
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

impl_color! { Rgb }
