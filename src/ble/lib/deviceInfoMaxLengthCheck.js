import { DEVICE_INFO_MAX_LENGTH } from '../constants';

export default info => (value) => {
  if (!value || value.length > DEVICE_INFO_MAX_LENGTH) {
    throw new Error((
      `\`${info}\` must be shorter than ${DEVICE_INFO_MAX_LENGTH} characters; ${value} is ${value ? value.length : 0}.`
    ));
  }
};
