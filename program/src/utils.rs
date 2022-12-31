use mpl_token_metadata::{
    assertions::collection::assert_is_collection_delegated_authority,
    state::{CollectionAuthorityRecord, Metadata},
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{error::MigrationError, state::MigrationState};

pub fn assert_valid_delegate(
    delegate_pubkey: &Pubkey,
    delegate_record_info: &AccountInfo,
    collection_metadata: &Metadata,
    migration_state: &MigrationState,
) -> Result<(), ProgramError> {
    // Mint is the correct one for the metadata account.
    if collection_metadata.mint != migration_state.collection_mint {
        return Err(MigrationError::MetadataMintMistmatch.into());
    }

    if collection_metadata.update_authority != migration_state.collection_authority {
        return Err(MigrationError::InvalidAuthority.into());
    }

    let bump = assert_is_collection_delegated_authority(
        delegate_record_info,
        delegate_pubkey,
        &migration_state.collection_mint,
    )?;

    let data = delegate_record_info.try_borrow_data()?;
    if data.len() == 0 {
        return Err(MigrationError::InvalidDelegate.into());
    }

    let record = CollectionAuthorityRecord::from_bytes(&data)?;

    if record.bump != bump {
        return Err(MigrationError::InvalidDelegate.into());
    }

    if let Some(update_authority) = record.update_authority {
        if update_authority != collection_metadata.update_authority {
            return Err(MigrationError::InvalidDelegate.into());
        }
    } else {
        return Err(MigrationError::InvalidDelegate.into());
    }

    Ok(())
}
