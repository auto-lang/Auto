//! 🍎 Application-specific utilities.

use std::ffi::CString;

use libc::pid_t;
use objc::runtime::{Class, Object};

use super::{CFObject, NonNull};

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
pub struct App(CFObject);

impl App {
    /// Returns the running application with the given process identifier, or
    /// `None` if no application has that pid.
    pub fn from_pid(pid: Pid) -> Option<App> {
        let cls: &Class = &NS_RUNNING_APPLICATION;
        unsafe { msg_send![cls, runningApplicationWithProcessIdentifier:pid] }
    }

    /// Returns whether the application is currently hidden.
    pub fn is_hidden(&self) -> bool {
        unsafe { msg_send![self.0.inner(), isHidden] }
    }

    /// Returns whether the application is currently frontmost.
    pub fn is_active(&self) -> bool {
        unsafe { msg_send![self.0.inner(), isActive] }
    }

    /// Attempts to activate the application using the specified options,
    /// returning whether or not it was successful.
    pub fn activate(&self, options: ActivationOptions) -> bool {
        unsafe { msg_send![self.0.inner(), activateWithOptions:options] }
    }

    /// Returns the localized name of the application. The value is suitable for
    /// presentation to the user.
    pub fn localized_name(&self) -> Option<CString> {
        // TODO: check whether this method leaks memory
        unsafe {
            let s = msg_send![self.0.inner(), localizedName];
            super::ns_string_encode_utf8(s)
        }
    }
}

bitflags! {
    /// Options to use when calling
    /// [`App::activate`](struct.App.html#method.activate).
    #[repr(C)]
    #[derive(Default)]
    pub struct ActivationOptions: usize {
        /// By default, activation brings only the main and key windows forward.
        /// With this option, all of the application's windows are brought
        /// forward.
        const ALL_WINDOWS = 1 << 0;
        /// By default, activation deactivates the calling app (assuming it was
        /// active), and then the new app is activated only if there is no
        /// currently active application. This prevents the new app from
        /// stealing focus from the user, if the app is slow to activate and the
        /// user has switched to a different app in the interim.
        ///
        /// However, with this option, the application is activated regardless
        /// of the currently active app, potentially stealing focus from the
        /// user. You should **rarely pass this flag** because stealing key
        /// focus produces a very bad user experience.
        const IGNORING_OTHER_APPS  = 1 << 1;
    }
}
