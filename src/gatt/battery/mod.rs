mod characteristic_battery_level;
mod descriptor_gatt_characteristic_presentation_format;
mod service_battery_service;

use self::{
    characteristic_battery_level::create_battery_level,
    descriptor_gatt_characteristic_presentation_format::create_gatt_characteristic_presentation_format,
    service_battery_service::create_battery_service,
};
use super::common::create_gatt_characteristic_user_description;
use bluster::gatt::service::Service;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::runtime::current_thread::Runtime;

pub fn create_battery(runtime: &Arc<Mutex<Runtime>>) -> Service {
    create_battery_service(true, {
        let mut characteristics = HashSet::new();
        characteristics.insert(create_battery_level(runtime, {
            let mut descriptors = HashSet::new();
            descriptors.insert(create_gatt_characteristic_user_description(
                runtime,
                String::from("Battery Level"),
            ));
            descriptors.insert(create_gatt_characteristic_presentation_format(
                runtime,
                (4, 1, 44327, 0, 0),
            ));
            descriptors
        }));
        characteristics
    })
}
