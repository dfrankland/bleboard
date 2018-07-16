import { Characteristic } from 'bleno';
import { CharacteristicUserDescriptionDescriptor } from '../descriptors';

const ProtocolModeCharacteristic = class extends Characteristic {
  constructor() {
    super({
      // org.bluetooth.characteristic.protocol_mode
      uuid: '2A4E',
      properties: ['read', 'writeWithoutResponse'],
      // secure: ['read', 'writeWithoutResponse'],
      descriptors: [
        new CharacteristicUserDescriptionDescriptor((
          'Protocol Mode'
        )),
      ],
    });
  }
};

export default ProtocolModeCharacteristic;
