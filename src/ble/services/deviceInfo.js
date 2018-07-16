import { PrimaryService } from 'bleno';
import {
  ManufacturerNameCharacteristic,
  ModelNumberCharacteristic,
  SerialNumberCharacteristic,
} from '../characteristics';

const DeviceInfoService = class extends PrimaryService {
  constructor({
    manufacturerName,
    modelNumber,
    serialNumber,
  }) {
    super({
      // org.bluetooth.service.device_information
      uuid: '180A',
      characteristics: [
        new ManufacturerNameCharacteristic(manufacturerName),
        new ModelNumberCharacteristic(modelNumber),
        new SerialNumberCharacteristic(serialNumber),
      ],
    });
  }
};

export default DeviceInfoService;
