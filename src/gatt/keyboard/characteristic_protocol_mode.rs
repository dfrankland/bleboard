use bluster::{
    gatt::{
        characteristic::{Characteristic, Properties, Read, Secure, Write},
        descriptor::Descriptor,
        event::{Event, Response},
    },
    SdpShortUuid,
};
use futures::{prelude::*, sync::mpsc::channel};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::runtime::current_thread::Runtime;
use uuid::Uuid;

pub fn create_protocol_mode(
    runtime: &Arc<Mutex<Runtime>>,
    descriptors: HashSet<Descriptor>,
    value_fields: (u8,),
) -> Characteristic {
    let (protocol_mode_value,) = value_fields;

    let value = vec![protocol_mode_value];
    let value2 = Arc::new(Mutex::new(value.clone()));
    let value3 = value.clone();

    let (sender, receiver) = channel(1);
    runtime
        .lock()
        .unwrap()
        .spawn(receiver.for_each(move |event| {
            let value = Arc::clone(&value2);
            match event {
                Event::ReadRequest(read_request) => {
                    read_request
                        .response
                        .send(Response::Success(value.lock().unwrap().clone()))
                        .unwrap();
                }
                Event::WriteRequest(write_request) => {
                    *value.lock().unwrap() = write_request.data;
                    write_request
                        .response
                        .send(Response::Success(vec![]))
                        .unwrap();
                }
                _ => {}
            }
            Ok(())
        }));

    Characteristic::new(
        Uuid::from_sdp_short_uuid(0x2A4C as u16), // org.bluetooth.characteristic.protocol_mode
        Properties::new(
            Some(Read(Secure::Secure(sender.clone()))),
            Some(Write::WithoutResponse(sender.clone())),
            None,
            None,
        ),
        Some(value3),
        descriptors,
    )
}
