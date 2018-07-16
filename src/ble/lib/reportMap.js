/* eslint-disable no-bitwise */

// Main items
export const INPUT = size => 0x80 | size;
export const OUTPUT = size => 0x90 | size;
export const COLLECTION = size => 0xA0 | size;
export const FEATURE = size => 0xB0 | size;
export const END_COLLECTION = size => 0xC0 | size;

// Global items
export const USAGE_PAGE = size => 0x04 | size;
export const LOGICAL_MINIMUM = size => 0x14 | size;
export const LOGICAL_MAXIMUM = size => 0x24 | size;
export const PHYSICAL_MINIMUM = size => 0x34 | size;
export const PHYSICAL_MAXIMUM = size => 0x44 | size;
export const UNIT_EXPONENT = size => 0x54 | size;
export const UNIT = size => 0x64 | size;
export const REPORT_SIZE = size => 0x74 | size;
export const REPORT_ID = size => 0x84 | size;
export const REPORT_COUNT = size => 0x94 | size;

// Local items
export const USAGE = size => 0x08 | size;
export const USAGE_MINIMUM = size => 0x18 | size;
export const USAGE_MAXIMUM = size => 0x28 | size;

export const LSB = value => value & 0xff;
export const MSB = value => (value >> 8) & 0xff;

export const REPORT_MAP = [
  // Generic Desktop Ctrls
  USAGE_PAGE(1),
  0x01,

  // Keyboard
  USAGE(1),
  0x06,

  // Application
  COLLECTION(1),
  0x01,

  // Application: Kbrd/Keypad
  USAGE_PAGE(1),
  0x07,

  USAGE_MINIMUM(1),
  0xE0,

  USAGE_MAXIMUM(1),
  0xE7,

  LOGICAL_MINIMUM(1),
  0x00,

  LOGICAL_MAXIMUM(1),
  0x01,

  // Application: 1 byte (Modifier)
  REPORT_SIZE(1),
  0x01,

  REPORT_COUNT(1),
  0x08,

  // Application: Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position
  INPUT(1),
  0x02,

  // Application: 1 byte (Reserved)
  REPORT_COUNT(1),
  0x01,

  REPORT_SIZE(1),
  0x08,

  // Application: Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position
  INPUT(1),
  0x01,

  // Application: 5 bits (Num lock, Caps lock, Scroll lock, Compose, Kana)
  REPORT_COUNT(1),
  0x05,

  REPORT_SIZE(1),
  0x01,

  // Application: LEDs
  USAGE_PAGE(1),
  0x08,

  // Application: Num Lock
  USAGE_MINIMUM(1),
  0x01,

  // Application: Kana
  USAGE_MAXIMUM(1),
  0x05,

  // Application: Data, Var, Abs, No Wrap, Linear, Preferred State,
  // No Null Position, Non-volatile
  OUTPUT(1),
  0x02,

  // Application: 3 bits (Padding)
  REPORT_COUNT(1),
  0x01,

  REPORT_SIZE(1),
  0x03,

  // Application: Const, Array, Abs, No Wrap, Linear, Preferred State,
  // No Null Position, Non-volatile
  OUTPUT(1),
  0x01,

  // Application: 6 bytes (Keys)
  REPORT_COUNT(1),
  0x06,

  REPORT_SIZE(1),
  0x08,

  LOGICAL_MINIMUM(1),
  0x00,

  // Application: 101 keys
  LOGICAL_MAXIMUM(1),
  0x65,

  // Application: Kbrd/Keypad
  USAGE_PAGE(1),
  0x07,

  USAGE_MINIMUM(1),
  0x00,

  USAGE_MAXIMUM(1),
  0x65,

  // Application: Data, Array, Abs, No Wrap, Linear, Preferred State,
  // No Null Position
  INPUT(1),
  0x00,

  END_COLLECTION(0),
];
