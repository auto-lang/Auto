//! Functionality for each individual operating system.

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(windows)]
pub mod windows;
