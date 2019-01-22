use super::report_types::ReportType;
use bluster::{
    gatt::{
        characteristic::{Characteristic, Properties, Read, Secure, Write},
        descriptor::Descriptor,
        event::{Event, Response},
    },
    SdpShortUuid,
};
use futures::{prelude::*, sync::mpsc::channel};
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
use std::{
    collections::HashSet,
    os::unix::io::RawFd,
    path::Path,
    sync::{atomic, Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::{runtime::current_thread::Runtime, timer::Interval};
use uuid::Uuid;

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

pub fn create_report(
    runtime: &Arc<Mutex<Runtime>>,
    descriptors: HashSet<Descriptor>,
    report_type: &ReportType,
) -> Characteristic {
    let mut runtime = runtime.lock().unwrap();

    // Spawn keylogging
    let (sender_key, receiver_key) = crossbeam::channel::unbounded();
    if let ReportType::InputReport = *report_type {
        let udev_context = udev::Context::new().unwrap();
        let mut libinput_context = Libinput::new_from_udev(LibinputInterfaceRaw, &udev_context);
        libinput_context
            .udev_assign_seat(&LibinputInterfaceRaw.seat())
            .unwrap();
        let input_stream = Interval::new_interval(Duration::from_millis(INPUT_SCAN_SPEED_MS))
            .map_err(|_| ())
            .for_each(move |_| {
                let sender_key = sender_key.clone();
                libinput_context.dispatch().unwrap();
                let mut keys = vec![];
                for event in libinput_context.clone() {
                    if let Keyboard(keyboard_event) = event {
                        let KeyboardEvent::Key(keyboard_key_event) = keyboard_event;
                        let key = keyboard_key_event.key();
                        keys.push(key);
                    }
                }
                sender_key.clone().try_send(keys).unwrap();
                Ok(())
            });
        runtime.spawn(input_stream);
    }

    let (sender, receiver) = channel(1);
    runtime.spawn(receiver.for_each(move |event| {
        let receiver_key = receiver_key.clone();
        let notifying = Arc::new(atomic::AtomicBool::new(false));
        match event {
            Event::ReadRequest(read_request) => {
                read_request
                    .response
                    .send(Response::Success(vec![]))
                    .unwrap();
            }
            Event::WriteRequest(write_request) => {
                // TODO: Logging of `write_request.data`?
                write_request
                    .response
                    .send(Response::Success(vec![]))
                    .unwrap();
            }
            Event::NotifySubscribe(notify_subscribe) => {
                let notifying = Arc::clone(&notifying);
                notifying.store(true, atomic::Ordering::Relaxed);
                thread::spawn(move || loop {
                    if !(&notifying).load(atomic::Ordering::Relaxed) {
                        break;
                    };

                    for keys in receiver_key.try_iter() {
                        for key in keys {
                            println!("Got key {:?}", key);
                            notify_subscribe
                                .clone()
                                .notification
                                .try_send(vec![key as u8])
                                .unwrap();
                        }
                    }
                });
            }
            Event::NotifyUnsubscribe => {
                notifying.store(false, atomic::Ordering::Relaxed);
            }
        };
        Ok(())
    }));

    let properties = match report_type {
        ReportType::InputReport => Properties::new(
            Some(Read(Secure::Secure(sender.clone()))),
            Some(Write::WithResponse(Secure::Secure(sender.clone()))),
            Some(sender.clone()),
            None,
        ),
        ReportType::OutputReport | ReportType::FeatureReport => Properties::new(
            Some(Read(Secure::Secure(sender.clone()))),
            Some(Write::WithResponse(Secure::Secure(sender.clone()))),
            None,
            None,
        ),
    };

    Characteristic::new(
        Uuid::from_sdp_short_uuid(0x2A4D as u16), // org.bluetooth.characteristic.report
        properties,
        Some(vec![]),
        descriptors,
    )
}
