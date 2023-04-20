use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum SecondOwnerError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Already Initialized")]
    AlreadyInitialized,

    #[error("Not The Expected Account Address")]
    NotExpectedAddress,

    #[error("Invalid String")]
    InvalidString,

    #[error("Wrong Account Owner")]
    WrongAccountOwner,

    #[error("Invalid Account Len")]
    InvalidAccountLen,
}

impl From<SecondOwnerError> for ProgramError {
    fn from(e: SecondOwnerError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
