use anchor_lang::prelude::*;
use xcall::program::Xcall;
use xcall::instructions::{admin::ReplyData,handle_request::ProxyReq};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Executea {
    pub contract_address: Pubkey,
    pub data: Vec<u8>,
}

// Declare the constants
pub const EXECUTE_NAME: &str = "Execute";
pub const CONFIGURE_PROTOCOLS_NAME: &str = "ConfigureProtocols";

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 1024 + 1024)]
    pub xcall_manager: Account<'info, XCallManager>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct XCallManager {
    pub xcall: Pubkey,
    pub icon_governance: String,
    pub sources: Vec<String>,
    pub destinations: Vec<String>,
    pub whitelisted_actions: Vec<Vec<u8>>,
    pub proposed_protocol_to_remove: String,
    pub admin: Pubkey,
    pub owner: Pubkey,
}

#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(mut)]
    pub xcall_manager: Account<'info, XCallManager>,
    pub admin: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Protocols {
    pub sources: Vec<String>,
    pub destinations: Vec<String>,
}

#[derive(Accounts)]
pub struct XCallGetter<'info> {
    pub xcall_manager: Account<'info, XCallManager>,
}

#[derive(Accounts)]
pub struct VerifyProtocolRecovery<'info> {
    pub xcall_manager: Account<'info, XCallManager>,
}

#[derive(Accounts)]
pub struct ExecuteCall<'info> {
    #[account(mut)]
    pub xcall_manager: Account<'info, XCallManager>,
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