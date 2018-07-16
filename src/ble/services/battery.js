import { PrimaryService } from 'bleno';
import { BatteryLevelCharacteristic } from '../characteristics';

const BatteryService = class extends PrimaryService {
  constructor() {
    super({
      // org.bluetooth.service.battery_service
      uuid: '180F',
      characteristics: [
        new BatteryLevelCharacteristic(),
      ],
    });
  }
};

export default BatteryService;
