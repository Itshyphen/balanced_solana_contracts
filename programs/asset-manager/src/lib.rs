use anchor_lang::prelude::*;
pub mod instructions;
pub mod error;
pub mod states;
pub mod structs;
pub mod helpers;
pub mod constant;

use anchor_lang::prelude::*;
use instructions::*;
use states::*;
use anchor_spl::token::{self, Token, Transfer, TokenAccount};

declare_id!("DXcb3bPmKdYJdQJ2cAKzWDexhgLG8d5RxTWCcXcAJAV4");

#[program]
pub mod asset_manager {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, xcall: Pubkey, icon_asset_manager: String, xcall_manager: Pubkey) -> Result<()> {
        instructions::initialize(ctx, xcall, icon_asset_manager, xcall_manager)
    }

    pub fn configure_rate_limit(
        ctx: Context<ConfigureRateLimit>,
        token: Pubkey,
        period: u64,
        percentage: u64,
    ) -> Result<()> {
        instructions::configure_rate_limit(ctx, token, period, percentage)
    }

    pub fn reset_limit(ctx: Context<ResetLimit>, token: Pubkey) -> Result<()> {
        instructions::reset_limit(ctx, token)
    }

    pub fn get_withdraw_limit(ctx: Context<GetWithdrawLimit>) -> Result<u64> {
        instructions::get_withdraw_limit(ctx)
    }

    pub fn deposit_native(ctx: Context<DepositNative>, amount: u64, to:Option<String>, data: Option<Vec<u8>>) -> Result<()> {
        // Transfer SOL
        instructions::deposit_native(ctx, amount, to, data)
    }

    pub fn deposit_token(ctx: Context<DepositToken>, amount: u64, to:Option<String>, data: Option<Vec<u8>>) -> Result<()> {
        // Transfer SPL Token
        instructions::deposit_token(ctx, amount, to, data)
    }

    pub fn execute_call(ctx: Context<ExecuteCall>, request_id: u128, data: Vec<u8>) -> Result<()> {
        instructions::execute_call(ctx, request_id, data)
    }

    pub fn execute_rollback(ctx: Context<ExecuteRollback>, sn: u128) -> Result<()> {
        instructions::execute_rollback(ctx, sn)
    }
}
