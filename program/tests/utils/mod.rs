use solana_program::pubkey::Pubkey;

pub fn find_migrate_state_pda(mint: Pubkey) -> Pubkey {
    let seeds = &[b"migration", mint.as_ref()];
    Pubkey::find_program_address(seeds, &mpl_migration_validator::ID).0
}
