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

pub fn create_hid_information(
    runtime: &Arc<Mutex<Runtime>>,
    descriptors: HashSet<Descriptor>,
    value_fields: (u16, u8, u8),
) -> Characteristic {
    let (bcd_hid, b_country_code, flags) = value_fields;
    let [bcd_hid_hi, bcd_hid_lo] = bcd_hid.to_le_bytes();

    let value = vec![bcd_hid_hi, bcd_hid_lo, b_country_code, flags];
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
        Uuid::from_sdp_short_uuid(0x2A4A as u16), // org.bluetooth.characteristic.hid_information
        Properties::new(Some(Read(Secure::Secure(sender.clone()))), None, None, None),
        Some(value2),
        descriptors,
    )
}
