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

pub fn create_gatt_characteristic_presentation_format(
    runtime: &Arc<Mutex<Runtime>>,
    value_fields: (u8, u8, u16, u8, u16),
) -> Descriptor {
    let (sender, receiver) = channel(1);
    let (format, exponent, unit, namespace, description) = value_fields;
    let [unit_hi, unit_lo] = unit.to_le_bytes();
    let [description_hi, description_lo] = description.to_le_bytes();
    let value = vec![
        format,
        exponent,
        unit_hi,
        unit_lo,
        namespace,
        description_hi,
        description_lo,
    ];
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
        Uuid::from_sdp_short_uuid(0x2901 as u16), // org.bluetooth.descriptor.gatt.characteristic_presentation_format
        Properties::new(Some(Read(Secure::Secure(sender))), None),
        Some(value2),
    )
}
