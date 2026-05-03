#![no_std]

use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};
use solana_program_log::log;

pub const ID: Address = Address::from_str_const("HtH3m4682j9Dq9bGx7K41fW7nT3PUTWi3dHbLUHY7ZYX");

#[cfg(not(feature = "no-entrypoint"))]
pinocchio::program_entrypoint!(process_instruction);
#[cfg(not(feature = "no-entrypoint"))]
pinocchio::default_allocator!();
#[cfg(not(feature = "no-entrypoint"))]
pinocchio::nostd_panic_handler!();

pub fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let payer = accounts
        .first()
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    if !payer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !payer.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    log!("Ping received by cpi_target!");
    Ok(())
}
