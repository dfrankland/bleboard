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
    sync::{Arc, Mutex},
};
use tokio::runtime::current_thread::Runtime;
use uuid::Uuid;

pub fn create_manufacturer_name_string(
    runtime: &Arc<Mutex<Runtime>>,
    descriptors: HashSet<Descriptor>,
    manufacturer_name: String,
) -> Characteristic {
    let value = Vec::from(manufacturer_name);
    let value2 = value.clone();

    let (sender, receiver) = channel(1);
    runtime
        .lock()
        .unwrap()
        .spawn(receiver.for_each(move |event| {
            let value = value.clone();
            if let Event::ReadRequest(read_request) = event {
                read_request
                    .response
                    .send(Response::Success(value))
                    .unwrap();
            }
            Ok(())
        }));

    Characteristic::new(
        Uuid::from_sdp_short_uuid(0x2A29 as u16), // org.bluetooth.characteristic.manufacturer_name_string
        Properties::new(Some(Read(Secure::Secure(sender.clone()))), None, None, None),
        Some(value2),
        descriptors,
    )
}
