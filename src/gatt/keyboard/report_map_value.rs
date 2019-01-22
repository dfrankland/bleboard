// Main items
fn input(size: u8) -> u8 {
    0x80 | size
}
fn output(size: u8) -> u8 {
    0x90 | size
}
fn collection(size: u8) -> u8 {
    0xA0 | size
}
// TODO: Needed?
// fn feature(size: u8) -> u8 {0xB0 | size}
fn end_collection(size: u8) -> u8 {
    0xC0 | size
}

// Global items
fn usage_page(size: u8) -> u8 {
    0x04 | size
}
fn logical_minimum(size: u8) -> u8 {
    0x14 | size
}
fn logical_maximum(size: u8) -> u8 {
    0x24 | size
}
// TODO: Needed?
// fn physical_minimum(size: u8) -> u8 {0x34 | size}
// fn physical_maximum(size: u8) -> u8 {0x44 | size}
// fn unit_exponent(size: u8) -> u8 {0x54 | size}
// fn unit(size: u8) -> u8 {0x64 | size}
fn report_size(size: u8) -> u8 {
    0x74 | size
}
// TODO: Needed?
// fn report_id(size: u8) -> u8 {0x84 | size}
fn report_count(size: u8) -> u8 {
    0x94 | size
}

// Local items
fn usage(size: u8) -> u8 {
    0x08 | size
}
fn usage_minimum(size: u8) -> u8 {
    0x18 | size
}
fn usage_maximum(size: u8) -> u8 {
    0x28 | size
}

// TODO: Needed?
// fn lsb(value: u8) -> u8 {value & 0xff}
// fn msb(value: u8) -> u8 {(value >> 8) & 0xff}

pub fn create_report_map_value() -> Vec<u8> {
    vec![
        // Generic Desktop Ctrls
        usage_page(1),
        0x01,
        // Keyboard
        usage(1),
        0x06,
        // Application
        collection(1),
        0x01,
        // Application: Kbrd/Keypad
        usage_page(1),
        0x07,
        usage_minimum(1),
        0xE0,
        usage_maximum(1),
        0xE7,
        logical_minimum(1),
        0x00,
        logical_maximum(1),
        0x01,
        // Application: 1 byte (Modifier)
        report_size(1),
        0x01,
        report_count(1),
        0x08,
        // Application: Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position
        input(1),
        0x02,
        // Application: 1 byte (Reserved)
        report_count(1),
        0x01,
        report_size(1),
        0x08,
        // Application: Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position
        input(1),
        0x01,
        // Application: 5 bits (Num lock, Caps lock, Scroll lock, Compose, Kana)
        report_count(1),
        0x05,
        report_size(1),
        0x01,
        // Application: LEDs
        usage_page(1),
        0x08,
        // Application: Num Lock
        usage_minimum(1),
        0x01,
        // Application: Kana
        usage_maximum(1),
        0x05,
        // Application: Data, Var, Abs, No Wrap, Linear, Preferred State,
        // No Null Position, Non-volatile
        output(1),
        0x02,
        // Application: 3 bits (Padding)
        report_count(1),
        0x01,
        report_size(1),
        0x03,
        // Application: Const, Array, Abs, No Wrap, Linear, Preferred State,
        // No Null Position, Non-volatile
        output(1),
        0x01,
        // Application: 6 bytes (Keys)
        report_count(1),
        0x06,
        report_size(1),
        0x08,
        logical_minimum(1),
        0x00,
        // Application: 101 keys
        logical_maximum(1),
        0x65,
        // Application: Kbrd/Keypad
        usage_page(1),
        0x07,
        usage_minimum(1),
        0x00,
        usage_maximum(1),
        0x65,
        // Application: Data, Array, Abs, No Wrap, Linear, Preferred State,
        // No Null Position
        input(1),
        0x00,
        end_collection(0),
    ]
}
