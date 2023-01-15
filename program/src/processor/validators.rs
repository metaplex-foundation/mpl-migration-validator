use super::*;

pub(crate) fn metadata_belongs_to_mint(
    metadata: &Metadata,
    mint: &Pubkey,
) -> Result<(), ProgramError> {
    if metadata.mint != *mint {
        return Err(ValidationError::MetadataMintMistmatch.into());
    }
    Ok(())
}

pub(crate) fn update_authority_matches(
    metadata: &Metadata,
    authority: &Pubkey,
) -> Result<(), ProgramError> {
    if metadata.update_authority != *authority {
        return Err(ValidationError::InvalidAuthority.into());
    }
    Ok(())
}

pub(crate) fn verified_collection_member(
    item_metadata: &Metadata,
    mint_pubkey: &Pubkey,
) -> Result<(), ProgramError> {
    if item_metadata.collection.is_none() {
        return Err(ValidationError::CollectionNotFound.into());
    }

    let collection = item_metadata.collection.as_ref().unwrap();

    if !collection.verified || collection.key != *mint_pubkey {
        return Err(ValidationError::NotCollectionMember.into());
    }
    Ok(())
}

pub(crate) fn metadata_derived_from_mint(
    metadata_info: &AccountInfo,
    mint_info: &AccountInfo,
) -> Result<(), ProgramError> {
    assert_derivation(
        &mpl_token_metadata::ID,
        metadata_info,
        &[
            PREFIX.as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            mint_info.key.as_ref(),
        ],
        ValidationError::MetadataMintMistmatch,
    )?;
    Ok(())
}

pub(crate) fn edition_derived_from_mint(
    edition_info: &AccountInfo,
    mint_info: &AccountInfo,
) -> Result<(), ProgramError> {
    assert_derivation(
        &mpl_token_metadata::ID,
        edition_info,
        &[
            PREFIX.as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            mint_info.key.as_ref(),
            EDITION.as_bytes(),
        ],
        ValidationError::InvalidEditionDerivation,
    )?;
    Ok(())
}

pub(crate) fn migration_state_derived_from_mint(
    migration_state_info: &AccountInfo,
    mint_info: &AccountInfo,
) -> Result<(), ProgramError> {
    assert_derivation(
        &crate::ID,
        migration_state_info,
        &[b"migration", mint_info.key.as_ref()],
        ValidationError::InvalidMigrationStateDerivation,
    )?;
    Ok(())
}

pub(crate) fn token_belongs_to_mint(
    token: &TokenAccount,
    mint_pubkey: &Pubkey,
) -> Result<(), ProgramError> {
    if token.mint != *mint_pubkey {
        return Err(ValidationError::TokenMintMismatch.into());
    }
    Ok(())
}

pub(crate) fn incoming_collection_mint_matches_stored(
    collection_mint_info: &AccountInfo,
    migration_state: &MigrationState,
) -> Result<(), ProgramError> {
    if migration_state.collection_info.mint != *collection_mint_info.key {
        return Err(ValidationError::CollectionMintMismatch.into());
    }
    Ok(())
}

pub(crate) fn incoming_collection_authority_matches_stored(
    collection_authority_info: &AccountInfo,
    migration_state: &MigrationState,
) -> Result<(), ProgramError> {
    if migration_state.collection_info.authority != *collection_authority_info.key {
        return Err(ValidationError::InvalidAuthority.into());
    }
    Ok(())
}
