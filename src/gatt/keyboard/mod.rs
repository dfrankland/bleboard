mod characteristic_hid_control_point;
mod characteristic_hid_information;
mod characteristic_protocol_mode;
mod characteristic_report;
mod characteristic_report_map;
mod descriptor_report_reference;
mod report_map_value;
mod report_types;
mod service_human_interface_device;

use self::{
    characteristic_hid_control_point::create_hid_control_point,
    characteristic_hid_information::create_hid_information,
    characteristic_protocol_mode::create_protocol_mode, characteristic_report::create_report,
    characteristic_report_map::create_report_map,
    descriptor_report_reference::create_report_reference,
    report_map_value::create_report_map_value, report_types::ReportType,
    service_human_interface_device::create_human_interface_device,
};
use bluster::gatt::service::Service;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::runtime::current_thread::Runtime;

pub fn create_keyboard(runtime: &Arc<Mutex<Runtime>>) -> Service {
    create_human_interface_device(true, {
        let mut characteristics = HashSet::new();
        characteristics.insert(create_hid_control_point(runtime, HashSet::new(), (0,)));
        characteristics.insert(create_hid_information(runtime, HashSet::new(), (273, 0, 3)));
        characteristics.insert(create_protocol_mode(runtime, HashSet::new(), (0,))); // TODO: Change this to 1 for report protocol mode and n-key rollover
        characteristics.insert({
            let report_type = &ReportType::InputReport;
            create_report(
                runtime,
                {
                    let mut descriptors = HashSet::new();
                    descriptors.insert(create_report_reference(runtime, (report_type,)));
                    descriptors
                },
                report_type,
            )
        });
        characteristics.insert({
            let report_type = &ReportType::OutputReport;
            create_report(
                runtime,
                {
                    let mut descriptors = HashSet::new();
                    descriptors.insert(create_report_reference(runtime, (report_type,)));
                    descriptors
                },
                report_type,
            )
        });
        characteristics.insert({
            let report_type = &ReportType::FeatureReport;
            create_report(
                runtime,
                {
                    let mut descriptors = HashSet::new();
                    descriptors.insert(create_report_reference(runtime, (report_type,)));
                    descriptors
                },
                report_type,
            )
        });
        characteristics.insert(create_report_map(
            runtime,
            HashSet::new(),
            (create_report_map_value(),),
        ));
        characteristics
    })
}
