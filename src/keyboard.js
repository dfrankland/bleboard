import hid from 'node-hid';

const {
  KEYBOARD_NAME = 'USB Gaming Keyboard', // Magicforce 68 Keyboard
} = process.env;

const hidDevices = hid.devices();

export default hidDevices.filter((
  ({ product }) => product === KEYBOARD_NAME
)).reduce(
  (keyboards, { path }) => {
    try {
      const device = new hid.HID(path);

      device.on('error', (data) => {
        console.error('Device error:', data); // eslint-disable-line no-console
      });

      return [...keyboards, device];
    } catch (err) {
      console.error('Error getting device:', err); // eslint-disable-line no-console
      return keyboards;
    }
  },
  [],
);
