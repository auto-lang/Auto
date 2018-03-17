//! Cross-platform input event automation facilities.
//!
//! This crate provides both high-level (any OS) and low level (per-OS)
//! functionality. See [the `os` module](./os/index.html) for APIs specific to
//! a certain operating system.

#![cfg_attr(all(test, nightly), feature(test))]

#![deny(missing_docs)]

#[cfg(all(test, nightly))]
extern crate test;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate lazy_static;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub mod os;

mod private {
    pub trait Priv {}
}
