import { PrimaryService } from 'bleno';
import {
  HidControlPointCharacteristic,
  HidInformationCharacteristic,
  ProtocolModeCharacteristic,
  ReportCharacteristic,
  ReportMapCharacteristic,
} from '../characteristics';
import {
  INPUT_REPORT,
  OUTPUT_REPORT,
  FEATURE_REPORT,
} from '../constants';
import { REPORT_MAP } from '../lib/reportMap';

const HumanInterfaceDeviceService = class extends PrimaryService {
  constructor({
    hidInformation = {
      bcdHid: [17, 1],
      bCountryCode: 0,
      flags: 3,
    },
    inputReport = true,
    outputReport = true,
    featureReport = false,
    inputReportCallback = (maxValueSize, updateValueCallback) => {
      updateValueCallback(Buffer.from([]));
    },
    outputReportCallback = data => console.log(data), // eslint-disable-line no-console
    reportMap = REPORT_MAP,
  }) {
    super({
      // org.bluetooth.service.human_interface_device
      uuid: '1812',
      characteristics: [
        new HidControlPointCharacteristic(),
        new HidInformationCharacteristic(hidInformation),
        new ProtocolModeCharacteristic(),
        ...(
          inputReport
            ? [new ReportCharacteristic({ reportType: INPUT_REPORT, inputReportCallback })]
            : []
        ),
        ...(
          outputReport
            ? [new ReportCharacteristic({ reportType: OUTPUT_REPORT, outputReportCallback })]
            : []
        ),
        ...(
          featureReport
            ? [new ReportCharacteristic({ reportType: FEATURE_REPORT })]
            : []
        ),
        new ReportMapCharacteristic(reportMap),
      ],
    });
  }
};

export default HumanInterfaceDeviceService;
