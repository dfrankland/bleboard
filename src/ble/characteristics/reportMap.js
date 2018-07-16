import { Characteristic } from 'bleno';
import { CharacteristicUserDescriptionDescriptor } from '../descriptors';

const ReportMapCharacteristic = class extends Characteristic {
  constructor(reportMap) {
    super({
      // org.bluetooth.characteristic.report_map
      uuid: '2A4B',
      properties: ['read'],
      // secure: ['read'],
      descriptors: [
        new CharacteristicUserDescriptionDescriptor((
          'Report Map'
        )),
      ],
      onReadRequest: (offset, callback) => {
        if (offset > reportMap.length) {
          callback(Characteristic.RESULT_INVALID_OFFSET, null);
          return;
        }

        callback(Characteristic.RESULT_SUCCESS, reportMap.slice(offset));
      },
    });
  }
};

export default ReportMapCharacteristic;
