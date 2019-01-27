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

pub fn create_report_map(
    runtime: &Arc<Mutex<Runtime>>,
    descriptors: HashSet<Descriptor>,
    value_fields: (Vec<u8>,),
) -> Characteristic {
    let (report_map_value,) = value_fields;
    let value = report_map_value;
    let value2 = value.clone();

    let (sender, receiver) = channel(1);
    runtime
        .lock()
        .unwrap()
        .spawn(receiver.for_each(move |event| {
            let value = value.clone();
            if let Event::ReadRequest(read_request) = event {
                let response = {
                    if read_request.offset > value.len() as u16 {
                        Response::InvalidOffset
                    } else {
                        Response::Success(value.split_at(read_request.offset as usize).1.to_vec())
                    }
                };

                read_request.response.send(response).unwrap();
            }
            Ok(())
        }));

    Characteristic::new(
        Uuid::from_sdp_short_uuid(0x2A4B as u16), // org.bluetooth.characteristic.report_map
        Properties::new(Some(Read(Secure::Secure(sender.clone()))), None, None, None),
        Some(value2),
        descriptors,
    )
}
