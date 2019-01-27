mod characteristic_manufacturer_name_string;
mod characteristic_model_number_string;
mod characteristic_serial_number_string;
mod service_device_information;

use self::{
    characteristic_manufacturer_name_string::create_manufacturer_name_string,
    characteristic_model_number_string::create_model_number_string,
    characteristic_serial_number_string::create_serial_number_string,
    service_device_information::create_device_information,
};
use bluster::gatt::service::Service;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::runtime::current_thread::Runtime;

pub fn create_device_info(runtime: &Arc<Mutex<Runtime>>) -> Service {
    create_device_information(true, {
        let mut characteristics = HashSet::new();
        characteristics.insert(create_manufacturer_name_string(
            runtime,
            HashSet::new(),
            String::from("bleboard"),
        ));
        characteristics.insert(create_model_number_string(
            runtime,
            HashSet::new(),
            String::from("6969"),
        ));
        characteristics.insert(create_serial_number_string(
            runtime,
            HashSet::new(),
            String::from("1337"),
        ));
        characteristics
    })
}
