import { Descriptor } from 'bleno';

const CharacteristicPresentationFormat = class extends Descriptor {
  constructor({
    format,
    exponent,
    unit,
    namespace,
    description,
  }) {
    super({
      uuid: '2904',
      value: Buffer.from([
        format,
        exponent,
        unit[0],
        unit[1],
        namespace,
        description[0],
        description[1],
      ]),
    });
  }
};

export default CharacteristicPresentationFormat;
