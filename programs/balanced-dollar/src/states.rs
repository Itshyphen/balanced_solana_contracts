use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::Metadata, token::{self, Mint, Token, TokenAccount, Transfer}};
use xcall_manager::program::XcallManager;
use xcall_manager::{XCallManager as XCallManagerConfig};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 40 + 40 + 8 + 8)]
    pub balanced_dollar: Account<'info, BalancedDollar>,
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: New Metaplex Account being created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"mint"],
        bump,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
}

#[derive(Accounts)]
pub struct CrossTransfer<'info> {
    pub sender: Signer<'info>,
    #[account(mut)]
    pub token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    pub config: Account<'info, BalancedDollar>,
    pub xcall_manager_config: Account<'info, XCallManagerConfig>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub fee: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ExecuteRollback<'info> {
    pub sender: Signer<'info>,
    #[account(mut)]
    pub config: Account<'info, BalancedDollar>,
    // #[account(mut)]
    // pub xcall: Account<'info, XCallState>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub treasury_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ExecuteCall<'info> {
    pub sender: Signer<'info>,
    #[account(mut)]
    pub config: Account<'info, BalancedDollar>,
    // #[account(mut)]
    // pub xcall: Account<'info, XCallState>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub treasury_authority: AccountInfo<'info>,
    #[account(mut)]
    pub fee: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]

pub struct BalancedDollar{
    pub xcall: Pubkey,
    pub icon_bnusd: String,
    pub xcall_manager: Pubkey,
    pub mint_authority: Pubkey,
    pub total_supply: u64,
}
