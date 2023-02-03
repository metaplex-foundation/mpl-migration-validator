/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';
import { InitializeArgs, initializeArgsBeet } from '../types/InitializeArgs';

/**
 * @category Instructions
 * @category Initialize
 * @category generated
 */
export type InitializeInstructionArgs = {
  initializeArgs: InitializeArgs;
};
/**
 * @category Instructions
 * @category Initialize
 * @category generated
 */
export const InitializeStruct = new beet.FixableBeetArgsStruct<
  InitializeInstructionArgs & {
    instructionDiscriminator: number;
  }
>(
  [
    ['instructionDiscriminator', beet.u8],
    ['initializeArgs', initializeArgsBeet],
  ],
  'InitializeInstructionArgs',
);
/**
 * Accounts required by the _Initialize_ instruction
 *
 * @property [_writable_, **signer**] payer Paying account for initiate migration
 * @property [**signer**] authority The collection authority
 * @property [] collectionMint The mint account of the collection parent NFT
 * @property [] collectionMetadata The metadata account of the collection parent NFT
 * @property [_writable_] migrationState The migration state account
 * @category Instructions
 * @category Initialize
 * @category generated
 */
export type InitializeInstructionAccounts = {
  payer: web3.PublicKey;
  authority: web3.PublicKey;
  collectionMint: web3.PublicKey;
  collectionMetadata: web3.PublicKey;
  migrationState: web3.PublicKey;
  systemProgram?: web3.PublicKey;
};

export const initializeInstructionDiscriminator = 0;

/**
 * Creates a _Initialize_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category Initialize
 * @category generated
 */
export function createInitializeInstruction(
  accounts: InitializeInstructionAccounts,
  args: InitializeInstructionArgs,
  programId = new web3.PublicKey('migrxZFChTqicHpNa1CAjPcF29Mui2JU2q4Ym7qQUTi'),
) {
  const [data] = InitializeStruct.serialize({
    instructionDiscriminator: initializeInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.payer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.authority,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.collectionMint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.collectionMetadata,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.migrationState,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
  ];

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  });
  return ix;
}