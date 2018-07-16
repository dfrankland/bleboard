import { Characteristic } from 'bleno';
import { CharacteristicUserDescriptionDescriptor } from '../descriptors';
import deviceInfoMaxLengthCheck from '../lib/deviceInfoMaxLengthCheck';

const ManufacturerNameCharacteristic = class extends Characteristic {
  constructor(manufacturerName) {
    deviceInfoMaxLengthCheck('manufacturerName')(manufacturerName);

    super({
      // org.bluetooth.characteristic.manufacturer_name_string
      uuid: '2A29',
      properties: ['read'],
      // secure: ['read'],
      value: Buffer.from(manufacturerName),
      descriptors: [
        new CharacteristicUserDescriptionDescriptor((
          'Manufacturer Name'
        )),
      ],
    });
  }
};

export default ManufacturerNameCharacteristic;
