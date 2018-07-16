import { Characteristic } from 'bleno';
import { CharacteristicUserDescriptionDescriptor } from '../descriptors';
import deviceInfoMaxLengthCheck from '../lib/deviceInfoMaxLengthCheck';

const SerialNumberCharacteristic = class extends Characteristic {
  constructor(serialNumber) {
    deviceInfoMaxLengthCheck('serialNumber')(serialNumber);

    super({
      // org.bluetooth.characteristic.serial_number_string
      uuid: '2A25',
      properties: ['read'],
      // secure: ['read'],
      value: Buffer.from(serialNumber),
      descriptors: [
        new CharacteristicUserDescriptionDescriptor((
          'Serial Number'
        )),
      ],
    });
  }
};

export default SerialNumberCharacteristic;
