use super::report_types::ReportType;
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

pub fn create_report_reference(
    runtime: &Arc<Mutex<Runtime>>,
    value_fields: (&ReportType,),
) -> Descriptor {
    let (report_type,) = value_fields;
    let value = vec![
        0,
        match report_type {
            ReportType::InputReport => 1,
            ReportType::OutputReport => 2,
            ReportType::FeatureReport => 3,
        },
    ];
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

    Descriptor::new(
        Uuid::from_sdp_short_uuid(0x2908 as u16), // org.bluetooth.descriptor.report_reference
        Properties::new(Some(Read(Secure::Secure(sender))), None),
        Some(value2),
    )
}
