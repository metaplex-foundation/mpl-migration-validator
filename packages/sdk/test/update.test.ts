import { Keypair, PublicKey } from '@solana/web3.js';
import spok from 'spok';
import test from 'tape';
import { InitializeArgs, UpdateArgs, MigrationState, MigrationType } from '../src/generated';
import { InitTransactions, killStuckProcess } from './setup';

killStuckProcess();

test('Update: successfully update state account', async (t) => {
  const API = new InitTransactions();
  const { fstTxHandler: handler, payerPair: payer, connection } = await API.payer();

  const defaultKey = new PublicKey('11111111111111111111111111111111');

  const { tx: tx1, mint } = await API.mintNft(handler, connection, payer, payer);
  await tx1.assertSuccess(t);

  const args: InitializeArgs = {
    ruleSet: defaultKey,
    migrationType: MigrationType.Timed,
  };

  const { tx: transaction, migrationState } = await API.initialize(
    handler,
    payer,
    payer,
    mint,
    args,
  );
  await transaction.assertSuccess(t);

  const state = await MigrationState.fromAccountAddress(connection, migrationState);
  spok(t, state, {
    collectionAuthority: payer.publicKey,
    collectionMint: mint,
    ruleSet: defaultKey,
    collectionDelegate: defaultKey,
    migrationType: args.migrationType,
    migrationSize: 0,
    inProgress: false,
    isEligible: false,
  });

  const newRuleSet = new Keypair().publicKey;

  console.log('newRuleSet', newRuleSet.toBase58());

  const updateArgs: UpdateArgs = {
    ruleSet: newRuleSet,
  };

  const { tx: updateTx } = await API.update(handler, payer, migrationState, updateArgs);
  await updateTx.assertSuccess(t);

  const newState = await MigrationState.fromAccountAddress(connection, migrationState);
  spok(t, newState, {
    collectionAuthority: payer.publicKey,
    collectionMint: mint,
    ruleSet: newRuleSet,
    collectionDelegate: defaultKey,
    migrationType: args.migrationType,
    migrationSize: 0,
    inProgress: false,
    isEligible: false,
  });
});
