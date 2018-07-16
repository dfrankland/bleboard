import { Characteristic } from 'bleno';
import {
  CharacteristicUserDescriptionDescriptor,
  ClientCharacterisiticConfigurationDescriptor,
  ReportReferenceDescriptor,
} from '../descriptors';
import {
  INPUT_REPORT,
  OUTPUT_REPORT,
  FEATURE_REPORT,
} from '../constants';

const REPORT_TYPES = {
  [INPUT_REPORT]: {
    properties: ['notify', 'read', 'write'],
    // secure: ['read', 'write'],
    descriptors: [
      new ClientCharacterisiticConfigurationDescriptor({ notifications: true }),
    ],
  },
  [OUTPUT_REPORT]: {
    properties: ['read', 'write', 'writeWithoutResponse'],
    // secure: ['read', 'write', 'writeWithoutResponse'],
  },
  [FEATURE_REPORT]: {
    properties: ['read', 'write'],
    // secure: ['read', 'write'],
  },
};

const ReportCharacteristic = class extends Characteristic {
  constructor({ reportType, outputReportCallback, inputReportCallback }) {
    const {
      properties,
      // secure,
      descriptors = [],
    } = REPORT_TYPES[reportType];

    super({
      // org.bluetooth.characteristic.report
      uuid: '2A4D',
      properties,
      // secure,
      descriptors: [
        new CharacteristicUserDescriptionDescriptor((
          'Report'
        )),
        new ReportReferenceDescriptor(properties),
        ...descriptors,
      ],
      onSubscribe: inputReportCallback,
      onWriteRequest: (data, offset, withoutResponse, callback) => {
        if (withoutResponse) return;
        if (reportType === OUTPUT_REPORT) outputReportCallback(data);
        callback(Characteristic.RESULT_SUCCESS, Buffer.from([]));
      },
      onReadRequest: (offset, callback) => {
        callback(Buffer.from([]));
      },
    });
  }
};

export default ReportCharacteristic;
