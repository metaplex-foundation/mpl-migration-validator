use borsh::BorshSerialize;
use mpl_token_metadata::{
    pda::find_metadata_account,
    state::{Metadata, MAX_METADATA_LEN},
};
use solana_program::{pubkey::Pubkey, stake_history::Epoch};
use solana_program_test::ProgramTest;
use solana_sdk::{account::Account, signature::Keypair, signer::Signer};

use crate::METADATA_RENT;

mod assert;
pub use assert::*;

pub fn find_migrate_state_pda(mint: Pubkey) -> (Pubkey, u8) {
    let seeds = &[b"migration", mint.as_ref()];
    Pubkey::find_program_address(seeds, &mpl_migration_validator::ID)
}

pub fn find_program_signer_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"signer"], &mpl_migration_validator::ID)
}

pub fn create_dummy_metadata_account(mint_pubkey: Pubkey, authority: Pubkey) -> Account {
    let mut metadata = Metadata::default();
    metadata.update_authority = authority;
    metadata.mint = mint_pubkey;

    let mut data = metadata.try_to_vec().unwrap();
    data.extend(vec![0; MAX_METADATA_LEN - data.len()]);

    Account {
        lamports: METADATA_RENT,
        data,
        owner: mpl_token_metadata::ID,
        executable: false,
        rent_epoch: Epoch::default(),
    }
}

pub fn set_up_dummy_init_context(test: &mut ProgramTest) {
    let mint_pubkey = Pubkey::new_unique();
    let metadata_pubkey = find_metadata_account(&mint_pubkey).0;
    let authority = Keypair::new();

    let metadata_account = create_dummy_metadata_account(mint_pubkey, authority.pubkey());

    test.add_account(metadata_pubkey, metadata_account);
}
