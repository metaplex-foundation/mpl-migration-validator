import { Amman } from '@metaplex-foundation/amman-client';
import { cusper } from '../utils/errors';

import { PROGRAM_ADDRESS } from '../../src/generated/src';
import { logDebug } from '.';

export const amman = Amman.instance({
  knownLabels: { [PROGRAM_ADDRESS]: 'Migration Validator Program' },
  log: logDebug,
  errorResolver: cusper,
});
