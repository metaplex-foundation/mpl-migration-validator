use super::*;

pub(crate) fn incoming_collection_authority_matches_stored(
    collection_authority_info: &AccountInfo,
    migration_state: &MigrationState,
) -> Result<(), ProgramError> {
    if migration_state.collection_info.authority != *collection_authority_info.key {
        return Err(MigrationError::InvalidAuthority.into());
    }
    Ok(())
}
