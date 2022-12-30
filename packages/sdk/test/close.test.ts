import { PublicKey } from '@solana/web3.js';
import spok from 'spok';
import test from 'tape';
import { InitializeArgs, MigrationState, MigrationType } from '../src/generated';
import { InitTransactions, killStuckProcess } from './setup';

killStuckProcess();

test('Close: successfully close migration state account', async (t) => {
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

  const { tx: closeTx } = await API.close(handler, payer, migrationState);
  await closeTx.assertSuccess(t);

  const account = await connection.getAccountInfo(migrationState);
  t.equal(account, null, 'account is null');
});
