use anchor_lang::prelude::*;
use crate::helpers::decode_execute_call_msg;
use crate::helpers::decode_method;
use crate::helpers::CONFIGURE_PROTOCOLS;
use crate::states::*;
use crate::error::*;
use xcall::cpi::accounts::ExecuteCall as XCallExecuteCall;


pub fn initialize(ctx: Context<Initialize>, xcall: Pubkey, icon_governance: String, sources: Vec<String>, destinations: Vec<String>) -> Result<()> {
    
    let xcall_manager = &mut ctx.accounts.xcall_manager;
    xcall_manager.xcall = xcall;
    xcall_manager.icon_governance = icon_governance;
    xcall_manager.sources = sources;
    xcall_manager.destinations = destinations;
    Ok(())
}

pub fn whitelist_action(ctx: Context<AdminAction>, action: Vec<u8>) -> Result<()> {
    let xcall_manager = &mut ctx.accounts.xcall_manager;
    xcall_manager.whitelisted_actions.push(action);
    Ok(())
}

pub fn remove_action(ctx: Context<AdminAction>, action: Vec<u8>) -> Result<()> {
    let xcall_manager = &mut ctx.accounts.xcall_manager;
    xcall_manager.whitelisted_actions.retain(|a| a != &action);
    Ok(())
}

pub fn propose_removal(ctx: Context<AdminAction>, protocol: String) -> Result<()> {
    let xcall_manager = &mut ctx.accounts.xcall_manager;
    xcall_manager.proposed_protocol_to_remove = protocol;
    Ok(())
}

pub fn set_admin(ctx: Context<AdminAction>, new_admin: Pubkey) -> Result<()> {
    let xcall_manager = &mut ctx.accounts.xcall_manager;
    require_keys_eq!(
        ctx.accounts.admin.key(),xcall_manager.admin,
        XCallManagerError::Unauthorized
    );
    xcall_manager.admin = new_admin;
    Ok(())
}

pub fn set_protocols(ctx: Context<AdminAction>, sources: Vec<String>, destinations: Vec<String>) -> Result<()> {
    let xcall_manager = &mut ctx.accounts.xcall_manager;
    require_keys_eq!(
        ctx.accounts.admin.key(),xcall_manager.admin,
        XCallManagerError::Unauthorized
    );
    xcall_manager.sources = sources;
    xcall_manager.destinations = destinations;
    Ok(())
}

pub fn get_protocols(ctx: Context<XCallGetter>) -> Result<Protocols> {
    let xcall_manager = &ctx.accounts.xcall_manager;
    Ok(Protocols {
        sources: xcall_manager.sources.clone(),
        destinations: xcall_manager.destinations.clone(),
    })
}

pub fn execute_call(
    ctx: Context<ExecuteCall>,
    _request_id: u128,
    _data: Vec<u8>,
) -> Result<()> {
    let xcall_manager = &mut ctx.accounts.xcall_manager;

    let cpi_program = ctx.accounts.xcall_program.to_account_info();
    let cpi_accounts = XCallExecuteCall {
        proxy_req: ctx.accounts.proxy_req.to_account_info(),
        reply_data: ctx.accounts.reply_data.to_account_info(),
        fee_handler: ctx.accounts.fee_handler.to_account_info(),
    };
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    let ticket = xcall::cpi::execute_call(cpi_context, _request_id, _data)?;

    let msg = vec![];
    let from = "".to_string();
    let ticket_protocols = vec!["".to_string()];

    let method = decode_method(&msg)?;

    if !verify_protocols_unordered(&ticket_protocols, &xcall_manager.sources) {
        require!(method==CONFIGURE_PROTOCOLS, XCallManagerError::ProtocolMismatch);
        verify_protocol_recovery(xcall_manager,&ticket_protocols);
    }

    let protocols = decode_execute_call_msg(&msg)?;

    xcall_manager.sources = protocols.sources;
    xcall_manager.destinations = protocols.destinations;

    Ok(())
}

fn verify_protocol_recovery(xcall_manager: &mut Account<XCallManager>,protocols: &Vec<String>) -> Result<()> {
    require!(xcall_manager.proposed_protocol_to_remove != "".to_string(), XCallManagerError::NoProposalForRemovalExists);
    let mut modified_sources = Vec::new();
    for source in xcall_manager.sources.clone() {
        if source != xcall_manager.proposed_protocol_to_remove {
            modified_sources.push(source);
        }
    }
    require!(verify_protocols_unordered(&modified_sources, &protocols), XCallManagerError::ProtocolMismatch);
    Ok(())
}

pub fn verify_protocols_unordered(array1: &[String], array2: &[String]) -> bool {
    if array1.len() != array2.len() {
        return false;
    }

    for item1 in array1 {
        let mut found = false;
        for item2 in array2 {
            if item1 == item2 {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    true
}