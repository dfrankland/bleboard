import { Descriptor } from 'bleno';

const getValue = (properties) => {
  const read = properties.indexOf('read') > -1;
  const write = properties.indexOf('write') > -1;
  const notify = properties.indexOf('notify') > -1;
  const writeWithoutResponse = properties.indexOf('writeWithoutResponse') > -1;

  // Input Report
  if (read && write && notify) return Buffer.from([0, 1]);

  // Output Report
  if (read && write && writeWithoutResponse) return Buffer.from([0, 2]);

  // Feature Report
  if (read && write) return Buffer.from([0, 3]);

  // Error
  return Buffer.from([]);
};

const ReportReferenceDescriptor = class extends Descriptor {
  constructor(properties) {
    super({
      // org.bluetooth.descriptor.report_reference
      uuid: '2908',
      value: getValue(properties),
    });
  }
};

export default ReportReferenceDescriptor;
