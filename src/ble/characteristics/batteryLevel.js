import { Characteristic } from 'bleno';
import batteryLevel from 'battery-level';
import { CharacteristicUserDescriptionDescriptor, CharacteristicPresentationFormat } from '../descriptors';

const BatteryLevelCharacteristic = class extends Characteristic {
  constructor() {
    super({
      // org.bluetooth.characteristic.battery_level
      uuid: '2A19',
      properties: ['read'],
      // secure: ['read'],
      descriptors: [
        new CharacteristicUserDescriptionDescriptor((
          'Battery Level'
        )),
        new CharacteristicPresentationFormat({
          format: 4,
          exponent: 1,
          unit: [39, 173],
          namespace: 0,
          description: [0, 0],
        }),
      ],
      onReadRequest: async (offset, callback) => {
        const batteryLevelPercentage = parseInt(await batteryLevel() * 100, 10);
        callback(Characteristic.RESULT_SUCCESS, Buffer.from([batteryLevelPercentage]));
      },
    });
  }
};

export default BatteryLevelCharacteristic;
