use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum FirstOwnerError {
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

impl From<FirstOwnerError> for ProgramError {
    fn from(e: FirstOwnerError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
