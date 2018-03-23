//! Cross-platform input event automation facilities.
//!
//! # Usage
//!
//! This crate is available [on crates.io](https://crates.io/crates/auto) and
//! can be used by adding the following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! auto = "0.0.8"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! extern crate auto;
//! ```
//!
//! # Cross-Platform Compatibility
//!
//! This crate provides both high-level (any OS) and low level (per-OS)
//! functionality:
//!
//! - The `os` module provides APIs specific to the current operating system
//!
//!   - [Linux](https://docs.rs/auto/0.0.8/x86_64-unknown-linux-gnu/auto/os/)
//!
//!   - [macOS](https://docs.rs/auto/0.0.8/x86_64-apple-darwin/auto/os/)
//!
//!   - [Windows](https://docs.rs/auto/0.0.8/x86_64-pc-windows-msvc/auto/os/)
//!
//! - All other modules work with any operating system
//!
//! [crate]: https://crates.io/crates/auto

#![cfg_attr(all(test, nightly), feature(test))]

#![allow(unknown_lints)]
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
extern crate libc;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "windows")]
extern crate winapi;

pub mod color;
pub mod os;

mod private {
    pub trait Priv {}
}
