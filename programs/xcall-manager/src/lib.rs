pub mod error;
pub mod instructions;
pub mod states;
pub mod helpers;

use anchor_lang::prelude::*;
use instructions::*;
pub use states::*;

declare_id!("Ejid4S3iZDVaBrFbnzhXmurUL28gNQqyMYHtmt3PtyJU");

#[program]
pub mod xcall_manager {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, xcall: Pubkey, icon_governance: String, sources: Vec<String>, destinations: Vec<String>) -> Result<()> {
        instructions::initialize(ctx, xcall, icon_governance, sources, destinations)
    }

    pub fn propose_removal(ctx: Context<AdminAction>, protocol: String) -> Result<()> {
        instructions::propose_removal(ctx, protocol)
    }

    pub fn whitelist_action(ctx: Context<AdminAction>, action: Vec<u8>) -> Result<()> {
        instructions::whitelist_action(ctx, action)
    }

    pub fn remove_action(ctx: Context<AdminAction>, action: Vec<u8>) -> Result<()> {
        instructions::remove_action(ctx, action)
    }

    pub fn set_admin(ctx: Context<AdminAction>, new_admin: Pubkey) -> Result<()> {
        instructions::set_admin(ctx, new_admin)
    }
    pub fn set_protocols(ctx: Context<AdminAction>, sources: Vec<String>, destinations: Vec<String>) -> Result<()> {
        instructions::set_protocols(ctx, sources, destinations)
    }

    pub fn get_protocols(ctx: Context<XCallGetter>) -> Result<Protocols>  {
        instructions::get_protocols(ctx)
    }

    pub fn verify_protocols(ctx: Context<XCallGetter>, protocols: Vec<String>) -> Result<bool> {
        let xcall_manager = &ctx.accounts.xcall_manager;
        Ok(verify_protocols_unordered(&protocols, &xcall_manager.sources))
    }

    pub fn execute_call(
        ctx: Context<ExecuteCall>,
        request_id: u128,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::execute_call(ctx, request_id, data)
    }
}


#[account]
pub struct ABCDEFG {
    pub xcall: Pubkey,
    pub icon_governance: String,
    pub sources: Vec<String>,
    pub destinations: Vec<String>,
    pub whitelisted_actions: Vec<Vec<u8>>,
}