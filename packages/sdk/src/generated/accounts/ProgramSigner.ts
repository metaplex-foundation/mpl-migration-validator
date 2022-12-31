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
 * Arguments used to create {@link ProgramSigner}
 * @category Accounts
 * @category generated
 */
export type ProgramSignerArgs = {
  bump: number;
};
/**
 * Holds the data for the {@link ProgramSigner} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class ProgramSigner implements ProgramSignerArgs {
  private constructor(readonly bump: number) {}

  /**
   * Creates a {@link ProgramSigner} instance from the provided args.
   */
  static fromArgs(args: ProgramSignerArgs) {
    return new ProgramSigner(args.bump);
  }

  /**
   * Deserializes the {@link ProgramSigner} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0,
  ): [ProgramSigner, number] {
    return ProgramSigner.deserialize(accountInfo.data, offset);
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link ProgramSigner} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig,
  ): Promise<ProgramSigner> {
    const accountInfo = await connection.getAccountInfo(address, commitmentOrConfig);
    if (accountInfo == null) {
      throw new Error(`Unable to find ProgramSigner account at ${address}`);
    }
    return ProgramSigner.fromAccountInfo(accountInfo, 0)[0];
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
    return beetSolana.GpaBuilder.fromStruct(programId, programSignerBeet);
  }

  /**
   * Deserializes the {@link ProgramSigner} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [ProgramSigner, number] {
    return programSignerBeet.deserialize(buf, offset);
  }

  /**
   * Serializes the {@link ProgramSigner} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return programSignerBeet.serialize(this);
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link ProgramSigner}
   */
  static get byteSize() {
    return programSignerBeet.byteSize;
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link ProgramSigner} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment,
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(ProgramSigner.byteSize, commitment);
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link ProgramSigner} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === ProgramSigner.byteSize;
  }

  /**
   * Returns a readable version of {@link ProgramSigner} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      bump: this.bump,
    };
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const programSignerBeet = new beet.BeetStruct<ProgramSigner, ProgramSignerArgs>(
  [['bump', beet.u8]],
  ProgramSigner.fromArgs,
  'ProgramSigner',
);
