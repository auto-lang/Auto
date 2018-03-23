#![cfg_attr(target_os = "linux",   doc = "🏎️ Linux-specific functionality.")]
#![cfg_attr(target_os = "macos",   doc = "🏎️ macOS-specific functionality.")]
#![cfg_attr(target_os = "windows", doc = "🏎️ Windows-specific functionality.")]
//!
//! Different documentations exist between
//! [**Linux**](https://docs.rs/auto/0.0.6/x86_64-unknown-linux-gnu/auto/os/),
//! [**macOS**](https://docs.rs/auto/0.0.6/x86_64-apple-darwin/auto/os/), and
//! [**Windows**](https://docs.rs/auto/0.0.6/x86_64-pc-windows-msvc/auto/os/).

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod imp;

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod imp;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod imp;

pub use self::imp::*;
