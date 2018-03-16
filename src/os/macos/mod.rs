//! macOS-specific functionality.

#![allow(non_snake_case)]

use objc::runtime::Class;
use objc::{Encode, Encoding};

#[link(name = "Cocoa", kind = "framework")]
extern {}

pub mod mouse;

lazy_static! {
    static ref NS_EVENT: &'static Class = Class::get("NSEvent").unwrap();
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CGPoint {
    x: f64,
    y: f64,
}

unsafe impl Encode for CGPoint {
    fn encode() -> Encoding {
        let inner = f64::encode();
        let encoding = format!("{{CGPoint={0}{0}}}", inner.as_str());
        unsafe { Encoding::from_str(&encoding) }
    }
}
