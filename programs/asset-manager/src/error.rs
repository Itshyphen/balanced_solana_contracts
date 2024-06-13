use anchor_lang::prelude::*;
use rlp::DecoderError;

#[error_code]
pub enum AssetManagerError {
    #[msg("Amount less than minimum amount")]
    AmountLessThanMinimum,
    #[msg("Exceeds withdraw limit")]
    ExceedsWithdrawLimit,
    #[msg("Failed to send tokens")]
    TokenTransferFailed,
    #[msg("Percentage should be less than or equal to 10000")]
    PercentageTooHigh,
    #[msg("Unauthorized caller")]
    UnauthorizedCaller,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Unknown Method")]
    UnknownMethod,
    #[msg("Method Decode Error")]
    DecoderError,
    #[msg("Invalid Instruction")]
    InvalidInstruction,
}

impl From<DecoderError> for AssetManagerError {
    fn from(_err: DecoderError) -> Self {
        AssetManagerError::DecoderError
    }
}


#[error_code]
pub enum CustomError {
    #[msg("Amount less than minimum amount")]
    AmountLessThanMinimum,
    #[msg("Exceeds withdraw limit")]
    ExceedsWithdrawLimit,
    #[msg("Failed to send tokens")]
    TokenTransferFailed,
    #[msg("Percentage should be less than or equal to 10000")]
    PercentageTooHigh,
    #[msg("Unauthorized caller")]
    UnauthorizedCaller,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Unknown Method")]
    UnknownMethod,
    #[msg("Method Decode Error")]
    DecodeError,
}
