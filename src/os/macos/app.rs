//! 🍎 Application-specific utilities.

use libc::pid_t;
use objc::runtime::Class;

lazy_static! {
    static ref NS_RUNNING_APPLICATION: &'static Class = {
        Class::get("NSRunningApplication").unwrap()
    };
}

/// A process identifier.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Pid(pub pid_t);

impl From<pid_t> for Pid {
    #[inline]
    fn from(pid: pid_t) -> Pid {
        Pid(pid)
    }
}

/// A running application.
#[derive(Debug)]
pub struct App(super::CFObject);

impl App {
    /// Returns the running application with the given process identifier, or
    /// `None` if no application has that pid.
    pub fn from_pid(pid: Pid) -> Option<App> {
        let cls: &Class = &NS_RUNNING_APPLICATION;
        unsafe { msg_send![cls, runningApplicationWithProcessIdentifier:pid] }
    }
}
