/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';

/**
 * @category Instructions
 * @category Start
 * @category generated
 */
export const StartStruct = new beet.BeetArgsStruct<{ instructionDiscriminator: number }>(
  [['instructionDiscriminator', beet.u8]],
  'StartInstructionArgs',
);
/**
 * Accounts required by the _Start_ instruction
 *
 * @property [_writable_, **signer**] payer Paying account for initiate migration
 * @property [**signer**] authority The collection authority
 * @property [] collectionMint The mint account of the collection parent NFT
 * @property [] collectionMetadata The metadata account of the collection parent NFT
 * @property [_writable_] delegateRecord The collection delegate record of for the program signer and the collection
 * @property [_writable_] migrationState The migration state account
 * @property [] splTokenProgram Token Program
 * @property [] tokenMetadataProgram Token Metadata program for the CPI call
 * @category Instructions
 * @category Start
 * @category generated
 */
export type StartInstructionAccounts = {
  payer: web3.PublicKey;
  authority: web3.PublicKey;
  collectionMint: web3.PublicKey;
  collectionMetadata: web3.PublicKey;
  delegateRecord: web3.PublicKey;
  migrationState: web3.PublicKey;
  splTokenProgram: web3.PublicKey;
  systemProgram?: web3.PublicKey;
  tokenMetadataProgram: web3.PublicKey;
};

export const startInstructionDiscriminator = 4;

/**
 * Creates a _Start_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category Start
 * @category generated
 */
export function createStartInstruction(
  accounts: StartInstructionAccounts,
  programId = new web3.PublicKey('migrxZFChTqicHpNa1CAjPcF29Mui2JU2q4Ym7qQUTi'),
) {
  const [data] = StartStruct.serialize({
    instructionDiscriminator: startInstructionDiscriminator,
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
      pubkey: accounts.delegateRecord,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.migrationState,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.splTokenProgram,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.tokenMetadataProgram,
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