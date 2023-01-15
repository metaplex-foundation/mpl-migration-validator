use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::{PrintProgramError, ProgramError},
    pubkey::Pubkey,
};

use crate::{errors::*, processor::Processor};

entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        match error {
            ProgramError::Custom(e) => match e {
                0..=24 => {
                    error.print::<GeneralError>();
                }
                25..=49 => {
                    error.print::<MigrationError>();
                }
                50..=74 => {
                    error.print::<ValidationError>();
                }
                75.. => {
                    error.print::<DeserializationError>();
                }
            },
            _ => {
                msg!(&error.to_string());
            }
        }
        return Err(error);
    }
    Ok(())
}
