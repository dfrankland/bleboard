import hid from 'node-hid';

const MAGIC_FORCE_PRODUCT = 'USB Gaming Keyboard';

const hidDevices = hid.devices();

const keyboards = hidDevices.filter((
  ({ product }) => product === MAGIC_FORCE_PRODUCT
));

keyboards.forEach((
  (keyboard) => {
    try {
      const device = new hid.HID(keyboard.path);

      device.on('data', (data) => {
        console.log('Data', data);
      });

      device.on('error', (data) => {
        console.log('Error', data);
      });

      console.log(device);
    } catch (err) {
      console.error(err);
    }
  }
));
