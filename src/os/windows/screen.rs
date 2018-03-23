//! 📺 Screen information utilities.

use std::ptr;

use winapi::um::wingdi;
use winapi::um::winuser;

use color::Rgb;

/// Returns the color on the screen at `x` and `y`.
pub fn color_at((x, y): (usize, usize)) -> Option<Rgb> {
    unsafe {
        let hdc = winuser::GetDC(ptr::null_mut());
        if hdc.is_null() {
            return None;
        }

        let color = wingdi::GetPixel(hdc, x as _, y as _);
        if color as usize == 0xFFFFFFFF {
            return None;
        }

        let r = wingdi::GetRValue(color) as u8;
        let g = wingdi::GetGValue(color) as u8;
        let b = wingdi::GetBValue(color) as u8;
        winuser::ReleaseDC(winuser::GetDesktopWindow(), hdc);

        Some(Rgb { red: r, green: g, blue: b })
    }
}

#[cfg(all(test, nightly))]
mod benches {
    use test::{Bencher, black_box};

    #[bench]
    fn color_at(b: &mut Bencher) {
        let loc = (0, 0);
        b.iter(|| {
            black_box(super::color_at(black_box(loc)));
        });
    }
}
