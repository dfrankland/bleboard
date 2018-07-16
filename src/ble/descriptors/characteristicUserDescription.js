import { Descriptor } from 'bleno';

const CharacteristicUserDescriptionDescriptor = class extends Descriptor {
  constructor(userDescription) {
    super({
      uuid: '2901',
      value: userDescription,
    });
  }
};

export default CharacteristicUserDescriptionDescriptor;
