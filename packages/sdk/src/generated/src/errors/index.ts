/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

type ErrorWithCode = Error & { code: number };
type MaybeErrorWithCode = ErrorWithCode | null | undefined;

const createErrorFromCodeLookup: Map<number, () => ErrorWithCode> = new Map();
const createErrorFromNameLookup: Map<string, () => ErrorWithCode> = new Map();

/**
 * MetadataMintMistmatch: 'Metadata does not match mint account'
 *
 * @category Errors
 * @category generated
 */
export class MetadataMintMistmatchError extends Error {
  readonly code: number = 0x0;
  readonly name: string = 'MetadataMintMistmatch';
  constructor() {
    super('Metadata does not match mint account');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, MetadataMintMistmatchError);
    }
  }
}

createErrorFromCodeLookup.set(0x0, () => new MetadataMintMistmatchError());
createErrorFromNameLookup.set('MetadataMintMistmatch', () => new MetadataMintMistmatchError());

/**
 * InvalidMetadata: 'Metadata did not deserialize correctly'
 *
 * @category Errors
 * @category generated
 */
export class InvalidMetadataError extends Error {
  readonly code: number = 0x1;
  readonly name: string = 'InvalidMetadata';
  constructor() {
    super('Metadata did not deserialize correctly');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidMetadataError);
    }
  }
}

createErrorFromCodeLookup.set(0x1, () => new InvalidMetadataError());
createErrorFromNameLookup.set('InvalidMetadata', () => new InvalidMetadataError());

/**
 * InvalidAuthority: 'Authority does not match the authority on the account'
 *
 * @category Errors
 * @category generated
 */
export class InvalidAuthorityError extends Error {
  readonly code: number = 0x2;
  readonly name: string = 'InvalidAuthority';
  constructor() {
    super('Authority does not match the authority on the account');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidAuthorityError);
    }
  }
}

createErrorFromCodeLookup.set(0x2, () => new InvalidAuthorityError());
createErrorFromNameLookup.set('InvalidAuthority', () => new InvalidAuthorityError());

/**
 * InvalidStateDerivation: 'Migration state account derivation is in correct'
 *
 * @category Errors
 * @category generated
 */
export class InvalidStateDerivationError extends Error {
  readonly code: number = 0x3;
  readonly name: string = 'InvalidStateDerivation';
  constructor() {
    super('Migration state account derivation is in correct');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidStateDerivationError);
    }
  }
}

createErrorFromCodeLookup.set(0x3, () => new InvalidStateDerivationError());
createErrorFromNameLookup.set('InvalidStateDerivation', () => new InvalidStateDerivationError());

/**
 * InvalidStateDeserialization: 'Migration state did not deserialize correctly'
 *
 * @category Errors
 * @category generated
 */
export class InvalidStateDeserializationError extends Error {
  readonly code: number = 0x4;
  readonly name: string = 'InvalidStateDeserialization';
  constructor() {
    super('Migration state did not deserialize correctly');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidStateDeserializationError);
    }
  }
}

createErrorFromCodeLookup.set(0x4, () => new InvalidStateDeserializationError());
createErrorFromNameLookup.set(
  'InvalidStateDeserialization',
  () => new InvalidStateDeserializationError(),
);

/**
 * MigrationInProgress: 'Cannot close while migration is in progress'
 *
 * @category Errors
 * @category generated
 */
export class MigrationInProgressError extends Error {
  readonly code: number = 0x5;
  readonly name: string = 'MigrationInProgress';
  constructor() {
    super('Cannot close while migration is in progress');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, MigrationInProgressError);
    }
  }
}

createErrorFromCodeLookup.set(0x5, () => new MigrationInProgressError());
createErrorFromNameLookup.set('MigrationInProgress', () => new MigrationInProgressError());

/**
 * IncorrectProgramOwner: 'Incorrect program owner for migration state account'
 *
 * @category Errors
 * @category generated
 */
export class IncorrectProgramOwnerError extends Error {
  readonly code: number = 0x6;
  readonly name: string = 'IncorrectProgramOwner';
  constructor() {
    super('Incorrect program owner for migration state account');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, IncorrectProgramOwnerError);
    }
  }
}

createErrorFromCodeLookup.set(0x6, () => new IncorrectProgramOwnerError());
createErrorFromNameLookup.set('IncorrectProgramOwner', () => new IncorrectProgramOwnerError());

/**
 * Overflow: 'Overflow error'
 *
 * @category Errors
 * @category generated
 */
export class OverflowError extends Error {
  readonly code: number = 0x7;
  readonly name: string = 'Overflow';
  constructor() {
    super('Overflow error');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, OverflowError);
    }
  }
}

createErrorFromCodeLookup.set(0x7, () => new OverflowError());
createErrorFromNameLookup.set('Overflow', () => new OverflowError());

/**
 * InvalidInstruction: 'Failed to build Migrate instruction'
 *
 * @category Errors
 * @category generated
 */
export class InvalidInstructionError extends Error {
  readonly code: number = 0x8;
  readonly name: string = 'InvalidInstruction';
  constructor() {
    super('Failed to build Migrate instruction');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidInstructionError);
    }
  }
}

createErrorFromCodeLookup.set(0x8, () => new InvalidInstructionError());
createErrorFromNameLookup.set('InvalidInstruction', () => new InvalidInstructionError());

/**
 * NoRuleSet: 'No rule set provided'
 *
 * @category Errors
 * @category generated
 */
export class NoRuleSetError extends Error {
  readonly code: number = 0x9;
  readonly name: string = 'NoRuleSet';
  constructor() {
    super('No rule set provided');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, NoRuleSetError);
    }
  }
}

createErrorFromCodeLookup.set(0x9, () => new NoRuleSetError());
createErrorFromNameLookup.set('NoRuleSet', () => new NoRuleSetError());

/**
 * InvalidSignerDerivation: 'Program signer account derivation is in correct'
 *
 * @category Errors
 * @category generated
 */
export class InvalidSignerDerivationError extends Error {
  readonly code: number = 0xa;
  readonly name: string = 'InvalidSignerDerivation';
  constructor() {
    super('Program signer account derivation is in correct');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidSignerDerivationError);
    }
  }
}

createErrorFromCodeLookup.set(0xa, () => new InvalidSignerDerivationError());
createErrorFromNameLookup.set('InvalidSignerDerivation', () => new InvalidSignerDerivationError());

/**
 * AlreadyInitialized: 'Program signer is already initialized'
 *
 * @category Errors
 * @category generated
 */
export class AlreadyInitializedError extends Error {
  readonly code: number = 0xb;
  readonly name: string = 'AlreadyInitialized';
  constructor() {
    super('Program signer is already initialized');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, AlreadyInitializedError);
    }
  }
}

createErrorFromCodeLookup.set(0xb, () => new AlreadyInitializedError());
createErrorFromNameLookup.set('AlreadyInitialized', () => new AlreadyInitializedError());

/**
 * InvalidDelegate: 'Invalid delegate'
 *
 * @category Errors
 * @category generated
 */
export class InvalidDelegateError extends Error {
  readonly code: number = 0xc;
  readonly name: string = 'InvalidDelegate';
  constructor() {
    super('Invalid delegate');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidDelegateError);
    }
  }
}

createErrorFromCodeLookup.set(0xc, () => new InvalidDelegateError());
createErrorFromNameLookup.set('InvalidDelegate', () => new InvalidDelegateError());

/**
 * FeatureDisabled: 'This feature is currently disabled'
 *
 * @category Errors
 * @category generated
 */
export class FeatureDisabledError extends Error {
  readonly code: number = 0xd;
  readonly name: string = 'FeatureDisabled';
  constructor() {
    super('This feature is currently disabled');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, FeatureDisabledError);
    }
  }
}

createErrorFromCodeLookup.set(0xd, () => new FeatureDisabledError());
createErrorFromNameLookup.set('FeatureDisabled', () => new FeatureDisabledError());

/**
 * Attempts to resolve a custom program error from the provided error code.
 * @category Errors
 * @category generated
 */
export function errorFromCode(code: number): MaybeErrorWithCode {
  const createError = createErrorFromCodeLookup.get(code);
  return createError != null ? createError() : null;
}

/**
 * Attempts to resolve a custom program error from the provided error name, i.e. 'Unauthorized'.
 * @category Errors
 * @category generated
 */
export function errorFromName(name: string): MaybeErrorWithCode {
  const createError = createErrorFromNameLookup.get(name);
  return createError != null ? createError() : null;
}