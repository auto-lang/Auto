#![cfg_attr(all(test, nightly), feature(test))]

#[cfg(all(test, nightly))]
extern crate test;

#[macro_use]
extern crate lazy_static;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub mod os;
