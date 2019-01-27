use bluster::{
    gatt::{characteristic::Characteristic, service::Service},
    SdpShortUuid,
};
use std::collections::HashSet;
use uuid::Uuid;

pub fn create_device_information(
    primary: bool,
    characteristics: HashSet<Characteristic>,
) -> Service {
    Service::new(
        Uuid::from_sdp_short_uuid(0x180A as u16), // org.bluetooth.service.device_information
        primary,
        characteristics,
    )
}
