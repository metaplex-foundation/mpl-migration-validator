import { initCusper } from '@metaplex-foundation/cusper';
import { errorFromCode } from '../../src/generated/src';

export const cusper = initCusper(errorFromCode);
