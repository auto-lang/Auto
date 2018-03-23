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
pub struct Rgb<T: RgbComponent = u8> {
    /// The amount of red.
    pub red: T,
    /// The amount of green.
    pub green: T,
    /// The amount of blue.
    pub blue: T,
}

impl_color! { Rgb }

macro_rules! forward_method {
    ($doc:expr, $meth:ident, $out:ty) => {
        #[doc = $doc]
        #[inline]
        pub fn $meth(self) -> Rgb<$out> {
            Rgb {
                red: self.red.$meth(),
                green: self.green.$meth(),
                blue: self.blue.$meth(),
            }
        }
    }
}

impl<T: RgbComponent> Rgb<T> {
    forward_method! {
        "Returns an RGB value with `f64` components between 0.0 and 1.0.",
        normalize, f64
    }

    forward_method! {
        "Returns an RGB value with `f64` components that are not guaranteed to
        be normalized",
        to_float, f64
    }

    forward_method! {
        "Returns an RGB value with `u8` components where 0 and 255 correspond to
        0.0 and 1.0 respectively.",
        to_byte, u8
    }
}

/// A type that can be used as a component of [`Rgb`](struct.Rgb.html).
pub trait RgbComponent {
    /// Normalizes `self` to a value between 0.0 and 1.0.
    fn normalize(self) -> f64 where Self: Sized;

    /// Returns `self` as a floating point value, not guaranteed to be
    /// normalized.
    #[inline]
    fn to_float(self) -> f64 where Self: Sized {
        self.normalize()
    }

    /// Returns `self` as a byte value
    fn to_byte(self) -> u8 where Self: Sized;
}

impl RgbComponent for u8 {
    #[inline]
    fn normalize(self) -> f64 {
        (self as f64) / 255.0
    }

    #[inline]
    fn to_byte(self) -> u8 {
        self
    }
}

impl RgbComponent for f64 {
    #[inline]
    fn normalize(self) -> f64 {
        self.max(0.0).min(1.0)
    }

    #[inline]
    fn to_float(self) -> f64 { self }

    #[inline]
    fn to_byte(self) -> u8 {
        (self.normalize() * 255.0) as u8
    }
}
