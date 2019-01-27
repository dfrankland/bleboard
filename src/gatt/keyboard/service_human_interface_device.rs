use bluster::{
    gatt::{characteristic::Characteristic, service::Service},
    SdpShortUuid,
};
use std::collections::HashSet;
use uuid::Uuid;

pub fn create_human_interface_device(
    primary: bool,
    characteristics: HashSet<Characteristic>,
) -> Service {
    Service::new(
        Uuid::from_sdp_short_uuid(0x1812 as u16), // org.bluetooth.service.human_interface_device
        primary,
        characteristics,
    )
}
