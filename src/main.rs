use futures::prelude::*;
use input::{
    event::{
        keyboard::{KeyboardEvent, KeyboardEventTrait},
        Event::Keyboard,
    },
    Libinput, LibinputInterface,
};
use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::close,
};
use std::{os::unix::io::RawFd, path::Path, time::Duration};
use tokio::{runtime::current_thread::Runtime, timer::Interval};

const INPUT_SCAN_SPEED_MS: u64 = 1;

struct LibinputInterfaceRaw;

impl LibinputInterfaceRaw {
    fn seat(&self) -> String {
        String::from("seat0")
    }
}

impl LibinputInterface for LibinputInterfaceRaw {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> std::result::Result<RawFd, i32> {
        if let Ok(fd) = open(path, OFlag::from_bits_truncate(flags), Mode::empty()) {
            Ok(fd)
        } else {
            Err(1)
        }
    }

    fn close_restricted(&mut self, fd: RawFd) {
        let _ = close(fd);
    }
}

fn main() {
    let udev_context = udev::Context::new().unwrap();
    let mut libinput_context = Libinput::new_from_udev(LibinputInterfaceRaw, &udev_context);
    libinput_context
        .udev_assign_seat(&LibinputInterfaceRaw.seat())
        .unwrap();

    let input_stream = Interval::new_interval(Duration::from_millis(INPUT_SCAN_SPEED_MS))
        .map_err(|_| ())
        .for_each(move |_| {
            libinput_context.dispatch().unwrap();
            for event in libinput_context.clone() {
                if let Keyboard(keyboard_event) = event {
                    let KeyboardEvent::Key(keyboard_key_event) = keyboard_event;
                    let key = keyboard_key_event.key();
                    println!("{}", key);
                }
            }
            Ok(())
        });

    let mut runtime = Runtime::new().unwrap();
    runtime.spawn(input_stream);
    runtime.run().unwrap();
}
