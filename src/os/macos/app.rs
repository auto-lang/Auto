//! 🍎 Application-specific utilities.

use std::ffi::CString;

use libc::pid_t;
use objc::runtime::{Class, Object};

use super::NSObject;

lazy_static! {
    static ref NS_RUNNING_APPLICATION: &'static Class = {
        Class::get("NSRunningApplication").unwrap()
    };

    static ref NS_STRING: &'static Class = {
        Class::get("NSString").unwrap()
    };

    static ref NS_WORKSPACE_SHARED: &'static Object = {
        let cls = Class::get("NSWorkspace").unwrap();
        unsafe { msg_send![cls, sharedWorkspace] }
    };

    static ref CURRENT_APPLICATION: App = {
        let cls: &Class = &NS_RUNNING_APPLICATION;
        unsafe { msg_send![cls, currentApplication] }
    };
}

fn str_to_ns_string(s: String) -> NSObject {
    let ns_string: &Class = &NS_STRING;
    let s = CString::new(s).unwrap();
    let utf8 = s.as_ptr();
    unsafe { msg_send![ns_string, stringWithUTF8String:utf8] }
}

/// Opens a file using the specified app.
///
/// The `appName` parameter need not be specified with a full path and, in the
/// case of an app wrapper, may be specified with or without the .app extension.
/// The sending app is deactivated before the request is sent.
pub fn open_file<'a, 'b, S>(path: &'a str, app_name: S)
    where S: Into<Option<&'b str>>
{
    let file = str_to_ns_string(path.into());
    let app  = app_name.into().map(|s| str_to_ns_string(s.into()));

    let workspace: &Object = &NS_WORKSPACE_SHARED;
    unsafe { msg_send![workspace, openFile:file withApplication:app] }
}

/// Launches the specified app, returning `true` on success or if it was already
/// running.
///
/// The appName parameter need not be specified with a full path and, in the
/// case of an app wrapper, may be specified with or without the .app extension.
pub fn launch(app: &str) -> bool {
    let app = str_to_ns_string(app.into());
    let workspace: &Object = &NS_WORKSPACE_SHARED;
    unsafe { msg_send![workspace, launchApplication:app] }
}

/// Terminates invisibly running, auto-terminable applications as if triggered
/// by system memory pressure.
///
/// This method corresponds to
/// `NSRunningApplication.terminateAutomaticallyTerminableApplications()`.
pub fn auto_terminate() {
    let cls: &Class = &NS_RUNNING_APPLICATION;
    unsafe { msg_send![cls, terminateAutomaticallyTerminableApplications] }
}

/// A process identifier.
pub type Pid = pid_t;

/// A running application.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct App(NSObject);

impl App {
    /// Returns the instance representing this application.
    pub fn current() -> &'static App {
        &CURRENT_APPLICATION
    }

    /// Returns the running application with the given process identifier, or
    /// `None` if no application has that pid.
    pub fn from_pid(pid: Pid) -> Option<App> {
        let cls: &Class = &NS_RUNNING_APPLICATION;
        unsafe { msg_send![cls, runningApplicationWithProcessIdentifier:pid] }
    }

    /// Returns the executing processor architecture for the application.
    pub fn arch(&self) -> Arch {
        unsafe { msg_send![self.0.inner(), executableArchitecture] }
    }

    /// Returns the process identifier of the application.
    pub fn pid(&self) -> Option<Pid> {
        match unsafe { msg_send![self.0.inner(), processIdentifier] } {
            -1 => None,
            id => Some(id),
        }
    }

    /// Returns whether the application is currently hidden.
    pub fn is_hidden(&self) -> bool {
        unsafe { msg_send![self.0.inner(), isHidden] }
    }

    /// Attempts to hide or unhide the application, returning whether the
    /// operation is successful. Returns `false` if the app has already quit, or
    /// if it is a type that is unable to be hidden.
    pub fn set_hidden(&self, hide: bool) -> bool {
        let app = self.0.inner();
        if hide {
            unsafe { msg_send![app, hide] }
        } else {
            unsafe { msg_send![app, unhide] }
        }
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

    /// Returns the `CFBundleIdentifier` of the application, or `None` if the
    /// application does not have an `Info.plist`.
    pub fn bundle_identifier(&self) -> Option<String> {
        unsafe {
            let s = msg_send![self.0.inner(), bundleIdentifier];
            super::ns_string_encode_utf8(s)
        }
    }

    /// Returns the URL to the application's bundle.
    pub fn bundle_url(&self) -> Option<String> {
        unsafe {
            let url = msg_send![self.0.inner(), bundleURL];
            super::ns_url_encode_utf8(url)
        }
    }

    /// Returns the URL to the application's executable.
    pub fn executable_url(&self) -> Option<String> {
        unsafe {
            let url = msg_send![self.0.inner(), executableURL];
            super::ns_url_encode_utf8(url)
        }
    }

    /// Returns the localized name of the application. The value is suitable for
    /// presentation to the user.
    pub fn localized_name(&self) -> Option<String> {
        unsafe {
            let s = msg_send![self.0.inner(), localizedName];
            super::ns_string_encode_utf8(s)
        }
    }

    /// Returns whether the application has terminated.
    pub fn is_terminated(&self) -> bool {
        unsafe { msg_send![self.0.inner(), isTerminated] }
    }

    /// Attempts to quit the application either forcefully or normally,
    /// returning whether the request is successful.
    pub fn terminate(&self, force: bool) -> bool {
        let app = self.0.inner();
        if force {
            unsafe { msg_send![app, forceTerminate] }
        } else {
            unsafe { msg_send![app, terminate] }
        }
    }

    /// Returns whether the application owns the current menu bar.
    pub fn owns_menu_bar(&self) -> bool {
        unsafe { msg_send![self.0.inner(), ownsMenuBar] }
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

/// An application architecture.
#[repr(usize)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Arch {
    /// The 32-bit Intel architecture.
    I386 = 0x00000007,
    /// The 32-bit PowerPC architecture.
    Ppc = 0x00000012,
    /// The 64-bit Intel architecture.
    X86_64 = 0x01000007,
    /// The 64-bit PowerPC architecture.
    Ppc64 = 0x01000012,
}

impl Arch {
    /// The architecture is 32-bit.
    #[inline]
    pub fn is_32bit(self) -> bool {
        !self.is_64bit()
    }

    /// The architecture is 64-bit.
    #[inline]
    pub fn is_64bit(self) -> bool {
        self as usize & 0x01000000 != 0
    }
}
