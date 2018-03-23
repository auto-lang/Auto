//! 🎨 Color utilities.

use std::hash;
use std::mem;
use std::slice;

const FLOAT_RATIO: f64 = 255.0;

macro_rules! impl_color {
    ($t:ident; $size:expr) => {
        impl<T> From<[T; $size]> for $t<T> {
            #[inline]
            fn from(arr: [T; $size]) -> Self {
                let val = unsafe { mem::transmute_copy(&arr) };
                mem::forget(arr);
                val
            }
        }

        impl<T> Into<[T; $size]> for $t<T> {
            #[inline]
            fn into(self) -> [T; $size] {
                let val = unsafe { mem::transmute_copy(&self) };
                mem::forget(self);
                val
            }
        }

        impl<T> AsRef<[T; $size]> for $t<T> {
            #[inline]
            fn as_ref(&self) -> &[T; $size] {
                let ptr = self as *const Self as *const [T; $size];
                unsafe { &*ptr }
            }
        }

        impl<T> AsMut<[T; $size]> for $t<T> {
            #[inline]
            fn as_mut(&mut self) -> &mut [T; $size] {
                let ptr = self as *mut Self as *mut [T; $size];
                unsafe { &mut *ptr }
            }
        }

        impl<T> AsRef<[T]> for $t<T> {
            #[inline]
            fn as_ref(&self) -> &[T] {
                AsRef::<[T; $size]>::as_ref(self)
            }
        }

        impl<T> AsMut<[T]> for $t<T> {
            #[inline]
            fn as_mut(&mut self) -> &mut [T] {
                AsMut::<[T; $size]>::as_mut(self)
            }
        }

        // Clippy lint
        #[allow(derive_hash_xor_eq)]
        impl hash::Hash for $t<u8> {
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
    ($t:ident) => {
        impl_color! { $t; mem::size_of::<$t<u8>>() }
    };
}

/// A simple ***Red-Green-Blue*** color triplet.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Rgb<T=u8> {
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
    /// Normalizes `self` in-place between `T`'s upper and lower bounds.
    #[inline]
    pub fn normalize(&mut self) {
        self.red.normalize();
        self.green.normalize();
        self.blue.normalize();
    }

    forward_method! {
        "Returns an RGB value with its components strictly between the lower and
        upper bounds.",
        normalized, T
    }

    forward_method! {
        "Returns an RGB value with `f64` components that are not guaranteed to
        be normalized.",
        into_float, f64
    }

    forward_method! {
        "Returns an RGB value with `u8` components where 0 and 255 correspond to
        0.0 and 1.0 respectively.

        Keep in mind that the `self` may not be normalized, which may result in
        strange results.",
        into_byte, u8
    }
}

/// A type that can be used as a component of [`Rgb`](struct.Rgb.html).
pub trait RgbComponent {
    /// Normalizes `self` in-place between `T`'s upper and lower bounds.
    fn normalize(&mut self);

    /// Returns `self` normalized between `T`'s upper and lower bounds.
    fn normalized(self) -> Self where Self: Sized;

    /// Returns `self` as a floating point value, not guaranteed to be
    /// normalized.
    fn into_float(self) -> f64 where Self: Sized;

    /// Returns `self` as a byte value. For reasonable results, it may be worth
    /// normalizing the value beforehand.
    fn into_byte(self) -> u8 where Self: Sized;
}

impl RgbComponent for u8 {
    #[inline]
    fn normalize(&mut self) {
        *self = self.normalized();
    }

    #[inline]
    fn normalized(self) -> u8 { self }

    #[inline]
    fn into_float(self) -> f64 { f64::from(self) / FLOAT_RATIO }

    #[inline]
    fn into_byte(self) -> u8 { self }
}

impl RgbComponent for f32 {
    #[inline]
    fn normalize(&mut self) {
        *self = self.normalized();
    }

    #[inline]
    fn normalized(self) -> f32 {
        self.max(0.0).min(1.0)
    }

    #[inline]
    fn into_float(self) -> f64 { self.into() }

    #[inline]
    fn into_byte(self) -> u8 {
        (self.normalized() * (FLOAT_RATIO as f32)) as u8
    }
}

impl RgbComponent for f64 {
    #[inline]
    fn normalize(&mut self) {
        *self = self.normalized();
    }

    #[inline]
    fn normalized(self) -> f64 {
        self.max(0.0).min(1.0)
    }

    #[inline]
    fn into_float(self) -> f64 { self }

    #[inline]
    fn into_byte(self) -> u8 {
        (self.normalized() * FLOAT_RATIO) as u8
    }
}
