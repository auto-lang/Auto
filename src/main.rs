#![allow(dead_code, unused_imports)]

extern crate auto;

use std::thread::sleep;
use std::time::Duration;

// use auto::os::app::{App, Pid, ActivationOptions};
use auto::os::mouse;

fn main() {
    mouse::warp_location((200.0, 400.0));

    // let pid = Pid::from(83045);
    // loop {
        // App::from_pid(pid).map(|app| {
        //     println!("{:?}", app.terminate());
        //     sleep(Duration::from_millis(70));
        // });
    // }
}
