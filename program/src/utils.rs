use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey, system_program,
};

pub fn close_program_account<'a>(
    account_info: &AccountInfo<'a>,
    funds_dest_account_info: &AccountInfo<'a>,
) -> ProgramResult {
    // Transfer lamports from the account to the destination account.
    let dest_starting_lamports = funds_dest_account_info.lamports();
    **funds_dest_account_info.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(account_info.lamports())
        .unwrap();
    **account_info.lamports.borrow_mut() = 0;

    // Realloc the account data size to 0 bytes and teassign ownership of
    // the account to the system program
    account_info.realloc(0, false)?;
    account_info.assign(&system_program::ID);

    Ok(())
}

pub fn find_migration_state_pda(mint: &Pubkey) -> (Pubkey, u8) {
    let seeds = &[b"migration", mint.as_ref()];
    Pubkey::find_program_address(seeds, &crate::ID)
}
