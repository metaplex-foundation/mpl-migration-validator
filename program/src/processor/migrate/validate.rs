use mpl_token_metadata::state::{EDITION, PREFIX};
use solana_program::program_option::COption;

use crate::utils::assert_valid_delegate;

use super::*;

pub(crate) fn validate_accounts(ctx: &AccountContext) -> Result<(), ProgramError> {
    // Signers
    assert_signer(ctx.payer)?;

    // Program ownership
    assert_owned_by(
        ctx.delegate_record,
        &mpl_token_metadata::ID,
        MigrationError::IncorrectProgramOwner,
    )?;

    assert_owned_by(
        ctx.mint,
        &SPL_TOKEN_ID,
        MigrationError::IncorrectProgramOwner,
    )?;
    assert_owned_by(
        ctx.metadata,
        &mpl_token_metadata::ID,
        MigrationError::IncorrectProgramOwner,
    )?;
    assert_owned_by(
        ctx.migration_state,
        ctx.program_id,
        MigrationError::IncorrectProgramOwner,
    )?;

    // Programs
    if ctx.token_metadata_program.key != &mpl_token_metadata::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    if ctx.system_program.key != &solana_program::system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    if ctx.spl_token_program.key != &SPL_TOKEN_ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    if ctx.sysvar_instructions.key != &sysvar::instructions::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}

pub(crate) fn validate_relationships(
    ctx: &AccountContext,
    data: &DataContext,
) -> Result<(), ProgramError> {
    // Collection NFT
    // The provided collection metadata must match the collection mint and update authority
    // stored on the migration state.
    if data.collection_metadata.mint != data.migration_state.collection_info.mint {
        return Err(MigrationError::MetadataMintMistmatch.into());
    }
    if data.collection_metadata.update_authority != data.migration_state.collection_info.authority {
        return Err(MigrationError::InvalidAuthority.into());
    }

    // Migration Item
    // The item's metadata and mint must match.
    if &data.metadata.mint != ctx.mint.key {
        return Err(MigrationError::MetadataMintMistmatch.into());
    }
    // The item's update authority must match that of the collection.
    if data.metadata.update_authority != data.migration_state.collection_info.authority {
        return Err(MigrationError::InvalidAuthority.into());
    }
    // The edition must be correctly derived from the mint.
    assert_derivation(
        ctx.program_id,
        ctx.edition,
        &[
            PREFIX.as_bytes(),
            ctx.program_id.as_ref(),
            ctx.mint.key.as_ref(),
            EDITION.as_bytes(),
        ],
        MigrationError::InvalidEditionDerivation,
    )?;

    // The token must belong to the mint
    if data.token.mint != *ctx.mint.key {
        return Err(MigrationError::InvalidTokenMint.into());
    }

    Ok(())
}

pub(crate) fn validate_eligibility(
    ctx: &AccountContext,
    data: &DataContext,
) -> Result<(), ProgramError> {
    // The Token Metadata edition PDA must have the freeze authority.
    if data.mint.freeze_authority != COption::Some(*ctx.edition.key) {
        return Err(MigrationError::IncorrectFreezeAuthority.into());
    }

    // The item metadata must be mutable.
    if !data.metadata.is_mutable {
        return Err(MigrationError::ImmutableMetadata.into());
    }

    if data.migration_state.collection_info.rule_set == Pubkey::default() {
        return Err(MigrationError::NoRuleSet.into());
    }

    Ok(())
}

pub(crate) fn validate_delegate(
    ctx: &AccountContext,
    data: &DataContext,
) -> Result<(), ProgramError> {
    // Validate that the delegate is the program signer for the correct
    // mint and update authority.
    assert_valid_delegate(
        ctx.program_signer.key,
        ctx.delegate_record,
        data.collection_metadata,
        data.migration_state,
    )?;

    Ok(())
}
