#![cfg(feature = "test-bpf")]
pub mod utils;

use borsh::{BorshDeserialize, BorshSerialize};
use mpl_migration_validator::{self, instruction::initiate};
use mpl_token_metadata::pda::find_metadata_account;
use mpl_token_metadata::state::{Metadata, MAX_METADATA_LEN};
use solana_program::{pubkey::Pubkey, stake_history::Epoch};
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{account::Account, signature::Keypair};
use solana_sdk::{signature::Signer, transaction::Transaction};
use utils::find_migrate_state_pda;

const METADATA_RENT: u64 = 5616720;

#[tokio::test]
async fn initiate_migration() {
    let mut test = ProgramTest::new("mpl_migration_validator", mpl_migration_validator::ID, None);

    let dummy_rule_set = Pubkey::new_unique();
    let mint_pubkey = Pubkey::new_unique();
    let metadata_pubkey = find_metadata_account(&mint_pubkey).0;
    let authority = Keypair::new();

    let migrate_state_pubkey = find_migrate_state_pda(mint_pubkey);

    let mut metadata = Metadata::default();
    metadata.update_authority = authority.pubkey();
    metadata.mint = mint_pubkey;

    let mut data = metadata.try_to_vec().unwrap();
    data.extend(vec![0; MAX_METADATA_LEN - data.len()]);

    let metadata_account = Account {
        lamports: METADATA_RENT,
        data,
        owner: mpl_token_metadata::ID,
        executable: false,
        rent_epoch: Epoch::default(),
    };

    test.add_account(metadata_pubkey, metadata_account);

    let mut context = test.start_with_context().await;

    let instruction = initiate(
        context.payer.pubkey(),
        authority.pubkey(),
        mint_pubkey,
        metadata_pubkey,
        migrate_state_pubkey,
        dummy_rule_set,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    let migrate_state_account = context
        .banks_client
        .get_account(migrate_state_pubkey)
        .await
        .unwrap()
        .unwrap();

    let migrate_state = mpl_migration_validator::state::MigrationState::try_from_slice(
        &migrate_state_account.data[..],
    )
    .unwrap();

    assert_eq!(migrate_state.collection(), mint_pubkey);
    assert_eq!(migrate_state.rule_set(), dummy_rule_set);
}
