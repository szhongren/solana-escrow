use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Exempt From Rent")]
    NotRentExempt,
    #[error("Expected Amount Does Not Match")]
    ExpectedAmountMismatch,
    #[error("Amount Overflow")]
    AmountOverflow,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
