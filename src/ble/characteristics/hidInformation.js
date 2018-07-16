import { Characteristic } from 'bleno';
import { CharacteristicUserDescriptionDescriptor } from '../descriptors';

const HidInformationCharacteristic = class extends Characteristic {
  constructor({
    bcdHid,
    bCountryCode,
    flags,
  }) {
    super({
      // org.bluetooth.characteristic.hid_information
      uuid: '2A4A',
      properties: ['read'],
      // secure: ['read'],
      value: Buffer.from([bcdHid[0], bcdHid[1], bCountryCode, flags]),
      descriptors: [
        new CharacteristicUserDescriptionDescriptor((
          'HID Information'
        )),
      ],
    });
  }
};

export default HidInformationCharacteristic;
