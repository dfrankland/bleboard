import { Descriptor } from 'bleno';

const ClientCharacterisiticConfigurationDescriptor = class extends Descriptor {
  constructor({ notifications, indications }) {
    super({
      // org.bluetooth.descriptor.gatt.client_characteristic_configuration
      uuid: '2902',

      // Notifications enabled
      value: Buffer.from([notifications ? 1 : 0, indications ? 1 : 0]),
    });
  }
};

export default ClientCharacterisiticConfigurationDescriptor;
