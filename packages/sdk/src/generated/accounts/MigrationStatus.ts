/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';
import * as beetSolana from '@metaplex-foundation/beet-solana';

/**
 * Arguments used to create {@link MigrationStatus}
 * @category Accounts
 * @category generated
 */
export type MigrationStatusArgs = {
  unlockTime: beet.bignum;
  isLocked: boolean;
  inProgress: boolean;
  itemsMigrated: number;
};
/**
 * Holds the data for the {@link MigrationStatus} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class MigrationStatus implements MigrationStatusArgs {
  private constructor(
    readonly unlockTime: beet.bignum,
    readonly isLocked: boolean,
    readonly inProgress: boolean,
    readonly itemsMigrated: number,
  ) {}

  /**
   * Creates a {@link MigrationStatus} instance from the provided args.
   */
  static fromArgs(args: MigrationStatusArgs) {
    return new MigrationStatus(args.unlockTime, args.isLocked, args.inProgress, args.itemsMigrated);
  }

  /**
   * Deserializes the {@link MigrationStatus} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0,
  ): [MigrationStatus, number] {
    return MigrationStatus.deserialize(accountInfo.data, offset);
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link MigrationStatus} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig,
  ): Promise<MigrationStatus> {
    const accountInfo = await connection.getAccountInfo(address, commitmentOrConfig);
    if (accountInfo == null) {
      throw new Error(`Unable to find MigrationStatus account at ${address}`);
    }
    return MigrationStatus.fromAccountInfo(accountInfo, 0)[0];
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey('migrxZFChTqicHpNa1CAjPcF29Mui2JU2q4Ym7qQUTi'),
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, migrationStatusBeet);
  }

  /**
   * Deserializes the {@link MigrationStatus} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [MigrationStatus, number] {
    return migrationStatusBeet.deserialize(buf, offset);
  }

  /**
   * Serializes the {@link MigrationStatus} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return migrationStatusBeet.serialize(this);
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link MigrationStatus}
   */
  static get byteSize() {
    return migrationStatusBeet.byteSize;
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link MigrationStatus} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment,
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(MigrationStatus.byteSize, commitment);
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link MigrationStatus} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === MigrationStatus.byteSize;
  }

  /**
   * Returns a readable version of {@link MigrationStatus} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      unlockTime: (() => {
        const x = <{ toNumber: () => number }>this.unlockTime;
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber();
          } catch (_) {
            return x;
          }
        }
        return x;
      })(),
      isLocked: this.isLocked,
      inProgress: this.inProgress,
      itemsMigrated: this.itemsMigrated,
    };
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const migrationStatusBeet = new beet.BeetStruct<MigrationStatus, MigrationStatusArgs>(
  [
    ['unlockTime', beet.i64],
    ['isLocked', beet.bool],
    ['inProgress', beet.bool],
    ['itemsMigrated', beet.u32],
  ],
  MigrationStatus.fromArgs,
  'MigrationStatus',
);
