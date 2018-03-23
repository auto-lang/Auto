//! Cross-platform input event automation facilities.
//!
//! This crate provides both high-level (any OS) and low level (per-OS)
//! functionality. See [the `os` module](./os/index.html) for APIs specific to
//! the current operating system.
//!
//! # Usage
//!
//! This crate is available [on crates.io](https://crates.io/crates/auto) and
//! can be used by adding the following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! auto = "0.0.6"
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
//! Auto aims to work on macOS, Windows, and Linux. This project is open to
//! adding more operating systems in the future.
//!
//! | Platform | Support |
//! | :------- | :------ |
//! | [Linux](https://docs.rs/auto/0.0.6/x86_64-unknown-linux-gnu/auto/os/) | **pending** |
//! | [macOS](https://docs.rs/auto/0.0.6/x86_64-apple-darwin/auto/os/) | **yes** |
//! | [Windows](https://docs.rs/auto/0.0.6/x86_64-pc-windows-msvc/auto/os/) | **pending** |
//!
//! [crate]: https://crates.io/crates/auto

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
extern crate libc;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "windows")]
extern crate winapi;

pub mod color;

#[cfg(target_os = "linux")]
#[path = "os/linux/mod.rs"]
pub mod os;

#[cfg(target_os = "macos")]
#[path = "os/macos/mod.rs"]
pub mod os;

#[cfg(target_os = "windows")]
#[path = "os/windows/mod.rs"]
pub mod os;

mod private {
    pub trait Priv {}
}
