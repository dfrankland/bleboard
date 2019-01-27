use bluster::{
    gatt::{characteristic::Characteristic, service::Service},
    SdpShortUuid,
};
use std::collections::HashSet;
use uuid::Uuid;

pub fn create_battery_service(primary: bool, characteristics: HashSet<Characteristic>) -> Service {
    Service::new(
        Uuid::from_sdp_short_uuid(0x180F as u16), // org.bluetooth.service.battery_service
        primary,
        characteristics,
    )
}
