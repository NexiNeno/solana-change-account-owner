use crate::*;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum FirstOwnerInstruction {
    Increase {},
    ChangeOwnership {},
    ZeroOutAccount {},
}

impl FirstOwnerInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpacking instruction...");

        let (tag, _) = input
            .split_first()
            .ok_or(errors::FirstOwnerError::InvalidInstruction)?;

        Ok(match tag {
            0 => Self::Increase {},
            1 => Self::ChangeOwnership {},
            2 => Self::ZeroOutAccount {},
            _ => return Err(errors::FirstOwnerError::InvalidInstruction.into()),
        })
    }
}
