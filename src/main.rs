#![feature(int_to_from_bytes)]

mod gatt;

use self::gatt::{create_battery, create_device_info, create_keyboard};
use bluster::Peripheral;
use futures::{future, prelude::*};
use std::sync::{Arc, Mutex};
use tokio::runtime::current_thread::Runtime;

const ADVERTISING_NAME: &str = "hello";

fn main() {
    let runtime = Arc::new(Mutex::new(Runtime::new().unwrap()));

    // Create peripheral
    let peripheral_future = Peripheral::new(&runtime);
    let peripheral = Arc::new({ runtime.lock().unwrap().block_on(peripheral_future).unwrap() });
    peripheral.add_service(&create_battery(&runtime)).unwrap();
    peripheral
        .add_service(&create_device_info(&runtime))
        .unwrap();
    peripheral.add_service(&create_keyboard(&runtime)).unwrap();

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
        let handled_advertising_stream = advertising_stream.for_each(|_| Ok(()));

        let advertising_check = future::loop_fn(Arc::clone(&peripheral), move |peripheral| {
            peripheral.is_advertising().and_then(move |is_advertising| {
                if is_advertising {
                    println!("Peripheral started advertising \"{}\"", ADVERTISING_NAME);
                    Ok(future::Loop::Break(peripheral))
                } else {
                    Ok(future::Loop::Continue(peripheral))
                }
            })
        });

        advertising_check.fuse().join(handled_advertising_stream)
    })
    .then(|_| Ok(()));

    // Spawn never ending process
    let mut runtime = runtime.lock().unwrap();
    runtime.spawn(advertisement);
    runtime.run().unwrap();
}
