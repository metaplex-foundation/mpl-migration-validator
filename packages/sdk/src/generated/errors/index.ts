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
 * InvalidAuthority: 'Authority does not match update authority on metadata'
 *
 * @category Errors
 * @category generated
 */
export class InvalidAuthorityError extends Error {
  readonly code: number = 0x2;
  readonly name: string = 'InvalidAuthority';
  constructor() {
    super('Authority does not match update authority on metadata');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidAuthorityError);
    }
  }
}

createErrorFromCodeLookup.set(0x2, () => new InvalidAuthorityError());
createErrorFromNameLookup.set('InvalidAuthority', () => new InvalidAuthorityError());

/**
 * InvalidStateAccount: 'Migration state account derivation is in correct'
 *
 * @category Errors
 * @category generated
 */
export class InvalidStateAccountError extends Error {
  readonly code: number = 0x3;
  readonly name: string = 'InvalidStateAccount';
  constructor() {
    super('Migration state account derivation is in correct');
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidStateAccountError);
    }
  }
}

createErrorFromCodeLookup.set(0x3, () => new InvalidStateAccountError());
createErrorFromNameLookup.set('InvalidStateAccount', () => new InvalidStateAccountError());

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
