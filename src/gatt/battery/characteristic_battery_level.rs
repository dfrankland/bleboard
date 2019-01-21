use bluster::{
    gatt::{
        characteristic::{Characteristic, Properties, Read, Secure},
        descriptor::Descriptor,
        event::{Event, Response},
    },
    SdpShortUuid,
};
use futures::{prelude::*, sync::mpsc::channel};
use std::{
    collections::HashSet,
    sync::{atomic, Arc, Mutex},
    thread,
    time::Duration,
};
use systemstat::{BatteryLife, Platform, System};
use tokio::runtime::current_thread::Runtime;
use uuid::Uuid;

fn battery_life_remaining_percentage(battery_life: &BatteryLife) -> u8 {
    let remaining_percentage = battery_life.remaining_capacity * 100.0;

    if remaining_percentage > 100.0 {
        return 100;
    }

    if remaining_percentage < 0.0 {
        return 0;
    }

    remaining_percentage as u8
}

pub fn create_battery_level(
    runtime: &Arc<Mutex<Runtime>>,
    descriptors: HashSet<Descriptor>,
) -> Characteristic {
    let (sender, receiver) = channel(1);
    runtime
        .lock()
        .unwrap()
        .spawn(receiver.for_each(move |event| {
            let notifying = Arc::new(atomic::AtomicBool::new(false));
            let sys = System::new();
            match event {
                Event::ReadRequest(read_request) => {
                    read_request
                        .response
                        .send(match sys.battery_life() {
                            Ok(battery_life) => {
                                Response::Success(vec![battery_life_remaining_percentage(
                                    &battery_life,
                                )])
                            }
                            Err(_) => Response::UnlikelyError,
                        })
                        .unwrap();
                }
                Event::NotifySubscribe(notify_subscribe) => {
                    let notifying = Arc::clone(&notifying);
                    notifying.store(true, atomic::Ordering::Relaxed);
                    thread::spawn(move || {
                        let mut remaining_capacity = 0;
                        loop {
                            if !(&notifying).load(atomic::Ordering::Relaxed) {
                                break;
                            };

                            let new_remaining_capacity = match sys.battery_life() {
                                Ok(battery_life) => {
                                    battery_life_remaining_percentage(&battery_life)
                                }
                                Err(_) => continue,
                            };

                            if new_remaining_capacity == remaining_capacity {
                                continue;
                            }

                            remaining_capacity = new_remaining_capacity;

                            notify_subscribe
                                .clone()
                                .notification
                                .try_send(vec![remaining_capacity])
                                .unwrap();

                            thread::sleep(Duration::from_secs(1));
                        }
                    });
                }
                Event::NotifyUnsubscribe => {
                    notifying.store(false, atomic::Ordering::Relaxed);
                }
                _ => {}
            };
            Ok(())
        }));
    Characteristic::new(
        Uuid::from_sdp_short_uuid(0x2A19 as u16), // org.bluetooth.characteristic.battery_level
        Properties::new(
            Some(Read(Secure::Secure(sender.clone()))),
            None,
            Some(sender.clone()),
            None,
        ),
        None,
        descriptors,
    )
}
