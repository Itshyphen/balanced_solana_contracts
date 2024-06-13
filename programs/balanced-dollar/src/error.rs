use anchor_lang::prelude::*;
use rlp::DecoderError;

#[error_code]
pub enum BalancedDollarError {
    #[msg("Amount less than minimum amount")]
    AmountLessThanMinimum,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Wrong Address")]
    WrongAddress,
    #[msg("Invalid Network Address according to Network ID")]
    InvalidNetworkAddress,
    #[msg("Wrong Network")]
    WrongNetwork,
    #[msg("Invalid to Address")]
    InvalidToAddress,
    #[msg("Token amount can't be zero")]
    InvalidAmount,
    #[msg("OnlyCallService")]
    OnlyCallService,
    #[msg("OnlyHub")]
    OnlyHub,
    #[msg("Invalid Method")]
    InvalidMethod,
    #[msg("Issue in Minting of Token")]
    MintError,
    #[msg("Issue in Burning of Token")]
    BurnError,
    #[msg("Invalid Data")]
    InvalidData,
    #[msg("Address Not Found")]
    AddressNotFound,
    #[msg("Cannot Send to Self")]
    CannotSendToSelf,
    #[msg("Decoder Error")]
    DecoderError,
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}

impl From<DecoderError> for BalancedDollarError {
    fn from(_err: DecoderError) -> Self {
        BalancedDollarError::DecoderError
    }
}