mod processor;
mod validate;

pub use processor::*;
use spl_token::state::{Account, Mint};
use validate::*;

use super::*;

pub(crate) struct AccountContext<'a> {
    pub(crate) program_id: &'a Pubkey,
    pub(crate) payer: &'a AccountInfo<'a>,
    pub(crate) metadata: &'a AccountInfo<'a>,
    pub(crate) edition: &'a AccountInfo<'a>,
    pub(crate) mint: &'a AccountInfo<'a>,
    pub(crate) delegate_record: &'a AccountInfo<'a>,
    pub(crate) migration_state: &'a AccountInfo<'a>,
    pub(crate) program_signer: &'a AccountInfo<'a>,
    pub(crate) system_program: &'a AccountInfo<'a>,
    pub(crate) sysvar_instructions: &'a AccountInfo<'a>,
    pub(crate) token_metadata_program: &'a AccountInfo<'a>,
    pub(crate) spl_token_program: &'a AccountInfo<'a>,
}

pub(crate) struct DataContext<'a> {
    pub(crate) metadata: &'a Metadata,
    pub(crate) collection_metadata: &'a Metadata,
    pub(crate) migration_state: &'a MigrationState,
    pub(crate) mint: &'a Mint,
    pub(crate) token: &'a Account,
}
