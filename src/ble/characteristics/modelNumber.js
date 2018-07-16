import { Characteristic } from 'bleno';
import { CharacteristicUserDescriptionDescriptor } from '../descriptors';
import deviceInfoMaxLengthCheck from '../lib/deviceInfoMaxLengthCheck';

const ModelNumberCharacteristic = class extends Characteristic {
  constructor(modelNumber) {
    deviceInfoMaxLengthCheck('modelNumber')(modelNumber);

    super({
      // org.bluetooth.characteristic.model_number_string
      uuid: '2A24',
      properties: ['read'],
      // secure: ['read'],
      value: Buffer.from(modelNumber),
      descriptors: [
        new CharacteristicUserDescriptionDescriptor((
          'Model Number'
        )),
      ],
    });
  }
};

export default ModelNumberCharacteristic;
