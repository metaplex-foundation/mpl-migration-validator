use borsh::BorshSerialize;
use mpl_migration_validator::{
    instruction::{initialize, start, update, InitializeArgs, UpdateArgs},
    state::{MigrationState, ProgramSigner, UnlockMethod},
};
use mpl_token_metadata::pda::find_collection_authority_account;
use solana_program::borsh::try_from_slice_unchecked;
use solana_program_test::{BanksClientError, ProgramTestContext};
use solana_sdk::{
    pubkey::Pubkey, signature::Signer, signer::keypair::Keypair, transaction::Transaction,
};

use super::*;

#[derive(Debug)]
pub struct Migratorr {
    pubkey: Pubkey,
    state: MigrationState,
}

impl Migratorr {
    pub fn new(collection_mint: Pubkey) -> Self {
        let (pubkey, _) = find_migrate_state_pda(&collection_mint);

        Migratorr {
            pubkey,
            state: MigrationState::default(),
        }
    }

    //      *****Getters*****         //
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    pub fn state(&self) -> &MigrationState {
        &self.state
    }

    pub fn authority(&self) -> Pubkey {
        self.state.collection_info.authority
    }

    pub fn mint(&self) -> Pubkey {
        self.state.collection_info.mint
    }

    pub fn delegate_record(&self) -> Pubkey {
        self.state.collection_info.delegate_record
    }

    pub fn rule_set(&self) -> Pubkey {
        self.state.collection_info.rule_set
    }

    pub fn collection_size(&self) -> u32 {
        self.state.collection_info.size
    }

    pub fn unlock_method(&self) -> UnlockMethod {
        self.state.unlock_method
    }

    //      *****Program Instructions*****         //
    pub async fn initialize(
        &self,
        context: &mut ProgramTestContext,
        payer: &Keypair,
        authority: &Keypair,
        nft: &NfTest,
        args: InitializeArgs,
    ) -> Result<(), BanksClientError> {
        let instruction = initialize(
            payer.pubkey(),
            authority.pubkey(),
            nft.mint_pubkey(),
            nft.metadata_pubkey(),
            self.pubkey,
            args,
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&*payer, &*authority],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(transaction).await
    }

    pub async fn update(
        &self,
        context: &mut ProgramTestContext,
        authority: &Keypair,
        vote_account: Option<Pubkey>,
        args: UpdateArgs,
    ) -> Result<(), BanksClientError> {
        let instruction = update(authority.pubkey(), self.pubkey, vote_account, args);

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&authority.pubkey()),
            &[&*authority],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(transaction).await
    }

    pub async fn start(
        &self,
        context: &mut ProgramTestContext,
        payer: &Keypair,
        authority: &Keypair,
        nft: &NfTest,
    ) -> Result<(), BanksClientError> {
        let delegate = ProgramSigner::pubkey();
        let (delegate_record, _) = find_collection_authority_account(&nft.mint_pubkey(), &delegate);

        let instruction = start(
            payer.pubkey(),
            authority.pubkey(),
            nft.mint_pubkey(),
            nft.metadata_pubkey(),
            delegate,
            delegate_record,
            self.pubkey,
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&*payer, &*authority],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(transaction).await
    }

    pub async fn close(
        &self,
        context: &mut ProgramTestContext,
        authority: &Keypair,
    ) -> Result<(), BanksClientError> {
        let instruction =
            mpl_migration_validator::instruction::close(authority.pubkey(), self.pubkey);

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&authority.pubkey()),
            &[&*authority],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(transaction).await
    }

    //      *****Utilities*****         //
    pub async fn refresh_state(
        &mut self,
        context: &mut ProgramTestContext,
    ) -> Result<(), BanksClientError> {
        let account = get_account(context, &self.pubkey).await;
        self.state = try_from_slice_unchecked(&account.data).unwrap();
        Ok(())
    }

    // Allows injecting a specific state into the on-chain
    // account. This is useful for testing the migration unlock time.
    pub async fn inject_state(&self, context: &mut ProgramTestContext, state: MigrationState) {
        let lamports = context
            .banks_client
            .get_account(self.pubkey())
            .await
            .unwrap()
            .unwrap()
            .lamports;

        let account = Account {
            lamports,
            data: state.try_to_vec().unwrap(),
            owner: mpl_migration_validator::ID,
            executable: false,
            rent_epoch: 0,
        };

        context.set_account(&self.pubkey(), &account.into())
    }
}
