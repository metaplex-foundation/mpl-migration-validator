use std::slice::Iter;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
};

pub(crate) struct AccountsValidator;

pub(crate) struct ValidatedAccount<'a> {
    account: &'a AccountInfo<'a>,
    accounts: &'a mut Iter<'a, AccountInfo<'a>>,
    seeds: &'a [&'a [u8]],
}

impl AccountsValidator {
    pub(crate) fn authority<'a>(
        accounts: &'a mut Iter<'a, AccountInfo<'a>>,
    ) -> Result<ValidatedAccount<'a>, ProgramError> {
        let authority = next_account_info(accounts)?;
        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(ValidatedAccount {
            account: authority,
            accounts,
            seeds: &[],
        })
    }
    // fn delegate(accounts: &[AccountInfo]) -> Result<ValidatedAccount, ProgramError> {
    //     let delegate = next_account_info(accounts)?;
    //     if !delegate.is_signer {
    //         return Err(ProgramError::MissingRequiredSignature);
    //     }
    //     Ok(ValidatedAccount {
    //         account: delegate,
    //         accounts,
    //     })
    // }
    // fn migration_state(accounts: &[AccountInfo]) -> Result<ValidatedAccount, ProgramError> {
    //     let migration_state = next_account_info(accounts)?;

    //     let bump = assert_derivation(
    //         program_id,
    //         migration_state_info,
    //         &[b"migration", collection_mint_info.key.as_ref()],
    //         MigrationError::InvalidStateDerivation,
    //     )?;
    //     Ok(ValidatedAccount {
    //         account: migration_state,
    //         accounts,
    //     })
    // }
}
