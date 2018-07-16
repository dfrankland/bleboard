import bleno from 'bleno';
import { services } from './ble';

const { DeviceInfoService, BatteryService, HumanInterfaceDeviceService } = services;

const advertisedServices = [
  new DeviceInfoService({
    manufacturerName: 'Cool',
    modelNumber: '6969',
    serialNumber: '1337',
  }),
  new BatteryService(),
  new HumanInterfaceDeviceService({}),
];

const advertisedServiceUuids = advertisedServices.map(({ uuid }) => uuid);

bleno.on('stateChange', (state) => {
  console.log('on -> stateChange:', state);

  if (state === 'poweredOn') {
    bleno.startAdvertising('bleboard', advertisedServiceUuids);
    return;
  }

  bleno.stopAdvertising();
});

bleno.on('advertisingStart', (advertisingStartErr) => {
  console.log('on -> advertisingStart:', ...(advertisingStartErr ? ['error', advertisingStartErr] : ['success']));

  if (advertisingStartErr) return;

  bleno.setServices(
    advertisedServices,
    (setServiceErr) => {
      console.log('setServices:', ...(setServiceErr ? ['error', setServiceErr] : ['success']));
    },
  );
});
