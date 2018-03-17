//! 🏎️ Functionality for each individual operating system.

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

/// The current operating system.
pub mod current {
    #[cfg(target_os = "linux")]
    pub use super::linux::*;

    #[cfg(target_os = "macos")]
    pub use super::macos::*;

    #[cfg(target_os = "windows")]
    pub use super::windows::*;
}
