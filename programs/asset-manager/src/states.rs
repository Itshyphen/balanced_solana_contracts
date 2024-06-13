use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer, TokenAccount};
use xcall::program::Xcall;
use xcall::instructions::{admin::ReplyData,handle_request::{ProxyReq, RollbackDataAccount} };

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + Config::INIT_SPACE)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(token: Pubkey)]
pub struct ConfigureRateLimit<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init, 
        seeds=[b"rate_limit", token.key().as_ref()], 
        space = 8 + RateLimit::INIT_SPACE, 
        payer = admin, 
        bump,
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub rate_limit: Account<'info, RateLimit>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResetLimit<'info> {
    #[account(has_one = admin)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub rate_limit: Account<'info, RateLimit>,
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetWithdrawLimit<'info> {
    #[account(has_one = admin)]
    pub config: Account<'info, Config>,
    pub rate_limit: Account<'info, RateLimit>,
    pub token_account: Account<'info, TokenAccount>,
    pub admin: Signer<'info>,
}


#[derive(Accounts)]
pub struct DepositToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub asset: Account<'info, Asset>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub config: Account<'info, Config>,
    // pub xcall_manager_program: Program<'info, XCallManager>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositNative<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub asset: Account<'info, Asset>,
    #[account(mut)]
    pub config: Account<'info, Config>,
    // pub xcall_manager_program: Program<'info, XCallManager>,
}

#[derive(Accounts)]
pub struct ExecuteCall<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub relayer: Signer<'info>,
    pub xcall_program: Program<'info, Xcall>,
    #[account(mut, close=fee_handler)]
    pub proxy_req: Account<'info, ProxyReq>,

    #[account(mut)]
    pub reply_data: Account<'info, ReplyData>,

    #[account(mut)]
    /// CHECK: Maybe needed
    pub fee_handler: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExecuteRollback<'info> {
    #[account(mut)]
    pub xcall_manager: Account<'info, Config>,
    #[account(mut)]
    pub relayer: Signer<'info>,
    pub xcall_program: Program<'info, Xcall>,
    #[account(mut)]
    pub rollback_data: Account<'info, RollbackDataAccount>,

    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub xcall: Pubkey,
    #[max_len(50)]
    pub icon_asset_manager: String,
    pub xcall_manager: Pubkey,
    pub admin: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct RateLimit {
    pub token: Pubkey,
    pub period: u64,
    pub percentage: u64,
    pub last_update: u64,
    pub current_limit: u64,
}

#[account]
pub struct Asset {
    pub asset_type: AssetType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AssetType {
    Native,
    Token,
}