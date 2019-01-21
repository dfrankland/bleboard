use bluster::{
    gatt::{
        descriptor::{Descriptor, Properties, Read, Secure},
        event::{Event, Response},
    },
    SdpShortUuid,
};
use futures::{prelude::*, sync::mpsc::channel};
use std::sync::{Arc, Mutex};
use tokio::runtime::current_thread::Runtime;
use uuid::Uuid;

pub fn create_gatt_characteristic_user_description(
    runtime: &Arc<Mutex<Runtime>>,
    description: String,
) -> Descriptor {
    let (sender, receiver) = channel(1);
    let value = Vec::from(description);
    let value2 = value.clone();

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

    Descriptor::new(
        Uuid::from_sdp_short_uuid(0x2901 as u16), // org.bluetooth.descriptor.gatt.characteristic_user_description
        Properties::new(Some(Read(Secure::Secure(sender))), None),
        Some(value2),
    )
}
