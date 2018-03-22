//! 📺 Screen information utilities.

use std::os::raw;
use std::ptr;

use libc::{boolean_t, size_t};
use objc::runtime::Class;

use super::{CGFloat, CGRect, CGSize, NSObject, NSObjectRef};
use color::Rgb;

extern {
    fn CGMainDisplayID() -> Display;

    fn CGGetOnlineDisplayList(
        max_displays: u32,
        online_displays: *mut Display,
        displayCount: *mut u32
    ) -> CGError;

    fn CGGetActiveDisplayList(
        max_displays: u32,
        online_displays: *mut Display,
        displayCount: *mut u32
    ) -> CGError;

    fn CGDisplayScreenSize(display: Display) -> CGSize;

    fn CGDisplayBounds(display: Display) -> CGRect;

    fn CGDisplayPixelsHigh(display: Display) -> size_t;

    fn CGDisplayPixelsWide(display: Display) -> size_t;

    fn CGDisplayIsActive(display: Display) -> boolean_t;

    fn CGDisplayIsAsleep(display: Display) -> boolean_t;

    fn CGDisplayIsBuiltin(display: Display) -> boolean_t;

    fn CGDisplayIsInMirrorSet(display: Display) -> boolean_t;

    fn CGDisplayIsInHWMirrorSet(display: Display) -> boolean_t;

    fn CGDisplayIsMain(display: Display) -> boolean_t;

    fn CGDisplayIsOnline(display: Display) -> boolean_t;

    fn CGDisplayIsStereo(display: Display) -> boolean_t;

    fn CGDisplayUsesOpenGLAcceleration(display: Display) -> boolean_t;

    fn CGDisplayModelNumber(display: Display) -> u32;

    fn CGDisplaySerialNumber(display: Display) -> u32;

    fn CGDisplayUnitNumber(display: Display) -> u32;

    fn CGDisplayVendorNumber(display: Display) -> u32;

    fn CGDisplayPrimaryDisplay(display: Display) -> Display;

    fn CGDisplayRotation(display: Display) -> f64;

    fn CGDisplayCreateImageForRect(display: Display, rect: CGRect) -> Option<CGImage>;

    fn CGImageRelease(image: CGImageRef);
}

lazy_static! {
    static ref NS_BITMAP: &'static Class = Class::get("NSBitmapImageRep").unwrap();
}

type CGError = i32;

type CGDisplayListGetter = unsafe extern fn(u32, *mut Display, *mut u32) -> CGError;

type CGImageRef = ptr::NonNull<raw::c_void>;

struct CGImage(CGImageRef);

impl Drop for CGImage {
    fn drop(&mut self) {
        unsafe { CGImageRelease(self.0) };
    }
}

/// The location and dimensions of a display.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bounds {
    /// Coordinates of the origin.
    pub origin: (f64, f64),
    /// Height and width of the bounds.
    pub size: (f64, f64),
}

impl From<CGRect> for Bounds {
    #[inline]
    fn from(rect: CGRect) -> Bounds {
        Bounds {
            origin: (rect.origin.x as _, rect.origin.y as _),
            size: (rect.size.width as _, rect.size.height as _),
        }
    }
}

/// A monitor display.
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Display(u32);

unsafe fn write_displays(get: CGDisplayListGetter, buf: &mut Vec<Display>) {
    buf.clear();
    let mut count = 0u32;
    if get(0, ptr::null_mut(), &mut count) == 0 {
        let len = count as usize;
        buf.reserve(len);
        if get(count, buf.as_mut_ptr(), &mut count) == 0 {
            buf.set_len(len);
        }
    }
}

impl Display {
    /// Returns the main display.
    ///
    /// The result is the display with its screen location at (0,0) in the
    /// global display coordinate space. In a system without display mirroring,
    /// the display with the menu bar is typically the main display.
    ///
    /// If mirroring is enabled and the menu bar appears on more than one
    /// display, this function provides a reliable way to find the main display.
    ///
    /// In case of hardware mirroring, the drawable display becomes the main
    /// display. In case of software mirroring, the display with the highest
    /// resolution and deepest pixel depth typically becomes the main display.
    #[inline]
    pub fn main() -> Display {
        unsafe { CGMainDisplayID() }
    }

    /// Returns all displays that are online (active, mirrored, or sleeping).
    ///
    /// If the framebuffer hardware is connected, a display is considered connected
    /// or online.
    ///
    /// When hardware mirroring is used, a display can be online but not active or
    /// drawable. Programs that manipulate display settings (such as gamma tables)
    /// need access to all displays, including hardware mirrors, which are not
    /// drawable.
    #[inline]
    pub fn online() -> Vec<Display> {
        let mut buf = Vec::new();
        Self::write_online(&mut buf);
        buf
    }

    /// Returns all displays that are active (or drawable).
    ///
    /// The first entry is the main display. In case of mirroring, the first
    /// entry is the largest drawable display or, if all are the same size, the
    /// display with the greatest pixel depth.
    ///
    /// Note that when hardware mirroring is being used between displays, only
    /// the primary display is active and appears in the list. When software
    /// mirroring is being used, all the mirrored displays are active and appear
    /// in the list.
    #[inline]
    pub fn active() -> Vec<Display> {
        let mut buf = Vec::new();
        Self::write_active(&mut buf);
        buf
    }

    /// Writes all online (active, mirrored, or sleeping) displays to `buf`
    /// after clearing it.
    pub fn write_online(buf: &mut Vec<Display>) {
        unsafe { write_displays(CGGetOnlineDisplayList, buf) };
    }

    /// Writes all active (or drawable) displays to `buf` after clearing it.
    pub fn write_active(buf: &mut Vec<Display>) {
        unsafe { write_displays(CGGetActiveDisplayList, buf) };
    }

    /// Returns whether the display is active.
    ///
    /// An active display is connected, awake, and available for drawing. In a
    /// hardware mirroring set, only the primary display is active.
    #[inline]
    pub fn is_active(self) -> bool {
        unsafe { CGDisplayIsActive(self) != 0 }
    }


    /// Returns whether the display is sleeping, and is therefore not drawable.
    #[inline]
    pub fn is_asleep(self) -> bool {
        unsafe { CGDisplayIsAsleep(self) != 0 }
    }

    /// Returns whether the display is built-in, such as the internal display in
    /// portable systems.
    ///
    /// Portable systems typically identify the internal LCD panel as a built-in
    /// display. Note that it is possible and reasonable for a system to have no
    /// displays marked as built-in. For example, a portable system running with
    /// the lid closed may report no built-in displays.
    #[inline]
    pub fn is_builtin(self) -> bool {
        unsafe { CGDisplayIsBuiltin(self) != 0 }
    }

    /// Returns whether the display is in a mirroring set (software or
    /// hardware).
    #[inline]
    pub fn in_mirror_set(self) -> bool {
        unsafe { CGDisplayIsInMirrorSet(self) != 0 }
    }

    /// Returns whether the display is in a hardware mirroring set.
    ///
    /// When hardware mirroring is enabled, the contents of a single framebuffer
    /// are rendered in all displays in the hardware mirroring set. All drawing
    /// operations are directed to the primary display in the set.
    #[inline]
    pub fn in_hw_mirror_set(self) -> bool {
        unsafe { CGDisplayIsInHWMirrorSet(self) != 0 }
    }

    /// Returns whether this display is the main display.
    #[inline]
    pub fn is_main(self) -> bool {
        unsafe { CGDisplayIsMain(self) != 0 }
    }

    /// Returns whether this display is connected or online.
    ///
    /// A display is considered connected or online when the framebuffer
    /// hardware is connected to a monitor.
    ///
    /// You can use this function to determine whether someone has plugged a
    /// display into the system while the main power was on. This hardware
    /// feature, called _hot-plugging_, may not be present on all displays.
    #[inline]
    pub fn is_online(self) -> bool {
        unsafe { CGDisplayIsOnline(self) != 0 }
    }

    /// Returns whether the display is running in a stereo graphics mode.
    #[inline]
    pub fn is_stereo(self) -> bool {
        unsafe { CGDisplayIsStereo(self) != 0 }
    }

    /// Returns whether Quartz is using OpenGL-based window acceleration (Quartz
    /// Extreme) to render in a display.
    ///
    /// Quartz Extreme is an OpenGL-based, hardware-accelerated window
    /// compositor available in macOS 10.2 and later. Quartz Extreme requires a
    /// minimum hardware configuration to operate.
    ///
    /// The information this function provides is typically used to adjust the
    /// demands of drawing operations to the capabilities of the display
    /// hardware. For example, an application running on an unaccelerated system
    /// could disable live window-resizing.
    #[inline]
    pub fn uses_open_gl(self) -> bool {
        unsafe { CGDisplayUsesOpenGLAcceleration(self) != 0 }
    }

    /// Returns the model number of the display's monitor.
    ///
    /// This function uses I/O Kit to identify the monitor associated with the
    /// specified display. The return value depends on the following:
    ///
    /// - If I/O Kit can identify the monitor, the product ID code for the
    ///   monitor is returned.
    ///
    /// - If I/O Kit can’t identify the monitor, `0x717`
    ///   (`kDisplayProductIDGeneric`) is returned.
    ///
    /// - If no monitor is connected, a value of `0xFFFFFFFF` is returned.
    #[inline]
    pub fn model_number(self) -> u32 {
        unsafe { CGDisplayModelNumber(self) }
    }

    ///
    ///
    /// This function uses I/O Kit to identify the monitor associated with the
    /// specified display.
    ///
    /// If I/O Kit can identify the monitor:
    ///
    /// - If the manufacturer has encoded a serial number for the monitor, the
    ///   number is returned.
    ///
    /// - If there is no encoded serial number, `0x00000000` is returned.
    ///
    /// If I/O Kit cannot identify the monitor:
    ///
    /// - If a monitor is connected to the display, `0x00000000` is returned.
    ///
    /// - If no monitor is connected to the display hardware, `0xFFFFFFFF` is
    ///   returned.
    ///
    /// Note that a serial number is meaningful only in conjunction with a
    /// specific vendor and product or model.
    #[inline]
    pub fn serial_number(self) -> u32 {
        unsafe { CGDisplaySerialNumber(self) }
    }

    /// Returns the logical unit number of the display.
    ///
    /// The logical unit number represents a particular node in the I/O Kit
    /// device tree associated with the display’s framebuffer.
    ///
    /// For a particular hardware configuration, this value will not change when
    /// the attached monitor is changed. The number will change, though, if the
    /// I/O Kit device tree changes, for example, when hardware is reconfigured,
    /// drivers are replaced, or significant changes occur to I/O Kit. Therefore
    /// keep in mind that this number may vary across login sessions.
    #[inline]
    pub fn unit_number(self) -> u32 {
        unsafe { CGDisplayUnitNumber(self) }
    }

    /// Returns the vendor number of the display's monitor.
    ///
    /// This function uses I/O Kit to identify the monitor associated with the
    /// specified display.
    ///
    /// - If I/O Kit can identify the monitor, the vendor ID is returned.
    ///
    /// - If I/O Kit cannot identify the monitor, kDisplayVendorIDUnknown is
    ///   returned.
    ///
    /// If there is no monitor associated with the display, `0xFFFFFFFF` is
    /// returned.
    #[inline]
    pub fn vendor_number(self) -> u32 {
        unsafe { CGDisplayVendorNumber(self) }
    }

    /// Returns the rotation angle of the display in degrees.
    #[inline]
    pub fn rotation(self) -> f64 {
        unsafe { CGDisplayRotation(self) }
    }

    /// Returns the color at the location relative to the origin of the display.
    pub fn color_at(self, pos: (f64, f64)) -> Option<Rgb> {
        self.colors(pos).next()
    }

    /// Returns an iterator over all colors at the location relative to the
    /// origin of the display.
    pub fn colors(self, pos: (f64, f64)) -> Colors {
        Colors::new(self, pos)
    }

    /// Returns the width and height of the display in millimeters, or 0 if the
    /// display is not valid.
    #[inline]
    pub fn size(self) -> (f64, f64) {
        let CGSize { width, height } = unsafe { CGDisplayScreenSize(self) };
        (width as _, height as _)
    }

    /// Returns the bounds of the display in the global coordinate space.
    #[inline]
    pub fn bounds(self) -> Bounds {
        unsafe { CGDisplayBounds(self).into() }
    }

    /// Returns the width and height in pixel units.
    #[inline]
    pub fn pixels(self) -> (usize, usize) {
        unsafe { (
            CGDisplayPixelsWide(self) as usize,
            CGDisplayPixelsHigh(self) as usize,
        ) }
    }

    /// Returns the primary display in a hardware mirroring set.
    #[inline]
    pub fn primary(self) -> Display {
        unsafe { CGDisplayPrimaryDisplay(self) }
    }
}

/// An iterator over colors on a display.
pub struct Colors {
    /// An `NSBitmapImageRep` instance.
    bitmap: NSObject,
    /// The display whose origin
    pub display: Display,
    /// An x-y position pair.
    pub pos: (f64, f64),
}

impl Iterator for Colors {
    type Item = Rgb;

    fn next(&mut self) -> Option<Rgb> {
        let (x, y) = self.pos;
        if !(x.is_finite() && y.is_finite()) {
            return None;
        }

        let disp   = self.display;
        let rect   = CGRect::new(x as _, y as _, 1.0, 1.0);
        let image  = unsafe { CGDisplayCreateImageForRect(disp, rect) }?;
        let bitmap = self.bitmap.inner();

        unsafe {
            let _: NSObjectRef = msg_send![bitmap, initWithCGImage:image];
            let c: NSObjectRef = msg_send![bitmap, colorAtX:0usize y:0usize];

            let mut r: CGFloat = 0.0;
            let mut g: CGFloat = 0.0;
            let mut b: CGFloat = 0.0;

            msg_send![
                c.as_ref(),
                getRed: (&mut r)
                green:  (&mut g)
                blue:   (&mut b)
                alpha:  ptr::null_mut::<CGFloat>()
            ];

            Some(Rgb { red: r as _, green: g as _, blue: b as _ })
        }
    }
}

impl From<(f64, f64)> for Colors {
    fn from(pos: (f64, f64)) -> Colors {
        Colors::new(Display::main(), pos)
    }
}

impl Colors {
    /// Creates a new instance for the display and position.
    pub fn new(display: Display, pos: (f64, f64)) -> Colors {
        let bitmap = NSObject::alloc(&NS_BITMAP);
        Colors { bitmap, display, pos }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_main() {
        assert!(Display::main().is_main());
    }

    #[test]
    fn display_primary() {
        let main = Display::main();
        assert_eq!(main, main.primary());
    }

    #[test]
    fn color_at() {
        use std::f64;

        let values  = [0.0, f64::NAN, f64::INFINITY];
        let display = Display::main();

        for &x in &values {
            for &y in &values {
                display.color_at((x, y));
            }
        }
    }
}

#[cfg(all(test, nightly))]
mod benches {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn main_100(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..100 {
                black_box(Display::main());
            }
        })
    }

    #[bench]
    fn size_100(b: &mut Bencher) {
        let display = Display::main();

        b.iter(|| {
            for _ in 0..100 {
                let (x, y) = black_box(display).size();
                black_box(x);
                black_box(y);
            }
        })
    }

    #[bench]
    fn online_100(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..100 {
                black_box(Display::online());
            }
        });
    }

    #[bench]
    fn write_online_100(b: &mut Bencher) {
        b.iter(|| {
            let mut buf = Vec::new();
            for _ in 0..100 {
                Display::write_online(black_box(&mut buf));
            }
        });
    }

    #[bench]
    fn color_at(b: &mut Bencher) {
        let display = Display::main();
        let loc = (0.0, 0.0);
        b.iter(|| {
            black_box(display.color_at(black_box(loc)));
        });
    }
}
