import { PublicKey } from '@solana/web3.js';
import spok from 'spok';
import test from 'tape';
import { InitializeArgs, MigrationState, UnlockMethod } from '../src/generated';
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
    migrationType: UnlockMethod.Timed,
    collectionSize: 0,
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
  spok(t, state.collectionInfo, {
    authority: payer.publicKey,
    mint: mint,
    ruleSet: defaultKey,
    delegate: defaultKey,
    size: 0,
  });
  spok(t, state.status, {
    inProgress: false,
    isLocked: true,
  });

  const { tx: closeTx } = await API.close(handler, payer, migrationState);
  await closeTx.assertSuccess(t);

  const account = await connection.getAccountInfo(migrationState);
  t.equal(account, null, 'account is null');
});
