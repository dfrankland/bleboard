import { Characteristic } from 'bleno';
import { CharacteristicUserDescriptionDescriptor } from '../descriptors';

const HidControlPointCharacteristic = class extends Characteristic {
  constructor() {
    super({
      // org.bluetooth.characteristic.hid_control_point
      uuid: '2A4C',
      properties: ['read', 'writeWithoutResponse'],
      // secure: ['read', 'writeWithoutResponse'],
      descriptors: [
        new CharacteristicUserDescriptionDescriptor((
          'HID Control Point'
        )),
      ],
      onReadRequest: (offset, callback) => {
        callback(Buffer.from([0]));
      },
    });
  }
};

export default HidControlPointCharacteristic;
