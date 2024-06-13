use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        self, close_account, transfer_checked, CloseAccount, Mint, Token, TokenAccount, Transfer,
        TransferChecked,
    },
};
use xcall_manager::program::XcallManager;
use xcall_manager::cpi::accounts::XCallGetter;
use xcall_manager::{self, Protocols};
use xcall::cpi::accounts::{ExecuteCall as XCallExecuteCall, Rollback as XCallExecuteRollback};


use crate::{
    constant::*, error::CustomError, helpers::*, states::*, structs::{
        deposit_message::DepositMessage,
        deposit_revert::{self, DepositRevert, DEPOSIT_REVERT}, withdraw_message::{WITHDRAW_TO, WITHDRAW_TO_NATIVE},
    }
};

const POINTS: u64 = 10000;
const NATIVE_ADDRESS: &str = "11111111111111111111111111111111";

pub fn initialize(
    ctx: Context<Initialize>,
    xcall: Pubkey,
    icon_asset_manager: String,
    xcall_manager: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.xcall = xcall;
    config.icon_asset_manager = icon_asset_manager;
    config.xcall_manager = xcall_manager;
    config.admin = ctx.accounts.admin.key();
    Ok(())
}

pub fn configure_rate_limit(
    ctx: Context<ConfigureRateLimit>,
    token: Pubkey,
    period: u64,
    percentage: u64,
) -> Result<()> {
    require!(percentage <= POINTS, CustomError::PercentageTooHigh);
    let limit_config = &mut ctx.accounts.rate_limit;

    let k = &ctx.accounts.token_account;
    let k_pubkey = k.owner;
    let (expected_key, _bumps) = Pubkey::find_program_address(
        &[b"rate_limit", k_pubkey.to_string().as_ref()],
        ctx.program_id,
    );
    let rate_limit_key = limit_config.key();
    require_keys_eq!(expected_key, rate_limit_key);

    limit_config.token = token;
    limit_config.period = period;
    limit_config.percentage = percentage;
    limit_config.last_update = Clock::get().unwrap().unix_timestamp as u64;
    limit_config.current_limit = k.amount * percentage / POINTS;
    Ok(())
}

pub fn reset_limit(ctx: Context<ResetLimit>, token: Pubkey) -> Result<()> {
    let limit_config = &mut ctx.accounts.rate_limit;
    Ok(())
}

pub fn get_withdraw_limit(ctx: Context<GetWithdrawLimit>) -> Result<u64> {
    let rate_limit = &ctx.accounts.rate_limit;
    let balance = ctx.accounts.token_account.amount;
    Ok(calculate_limit(balance, rate_limit))
}

pub fn deposit_native(
    ctx: Context<DepositNative>,
    amount: u64,
    to: Option<String>,
    data: Option<Vec<u8>>,
) -> Result<()> {
    // Transfer SOL
    require!(amount > 0, CustomError::InvalidAmount);
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user.key(),
        &ctx.accounts.asset.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.asset.to_account_info(),
        ],
    )?;

    let fee = ctx.accounts.user.lamports() - amount;
    let from: Pubkey = ctx.accounts.user.key();
    send_deposit_message(
        &ctx.accounts.config,
        NATIVE_ADDRESS.to_owned(),
        from,
        amount,
        to,
        data,
        fee,
    );
    Ok(())
}

pub fn deposit_token(
    ctx: Context<DepositToken>,
    amount: u64,
    to: Option<String>,
    data: Option<Vec<u8>>,
) -> Result<()> {
    // Transfer SPL Token
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    let fee = ctx.accounts.user.lamports();
    let token_addr = ctx.accounts.token_program.key().to_string();
    let from: Pubkey = ctx.accounts.user.key();
    send_deposit_message(
        &ctx.accounts.config,
        token_addr,
        from,
        amount,
        to,
        data,
        fee,
    );
    Ok(())
}

pub fn execute_rollback(ctx: Context<ExecuteRollback>, sn: u128) -> Result<()> {
    let cpi_program = ctx.accounts.xcall_program.to_account_info();
    let cpi_accounts = XCallExecuteRollback {
        rollback_data: ctx.accounts.rollback_data.to_account_info(),
        signer: ctx.accounts.signer.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    let ticket = xcall::cpi::execute_rollback(cpi_context, sn)?;

    let data: Vec<u8>= vec![];
    let deposit_revert = decode_deposit_revert_msg(&data)?;


    Ok(())
}

pub fn execute_call(ctx: Context<ExecuteCall>, _request_id: u128, _data: Vec<u8>) -> Result<()> {
    let cpi_program = ctx.accounts.xcall_program.to_account_info();
    let cpi_accounts = XCallExecuteCall {
        proxy_req: ctx.accounts.proxy_req.to_account_info(),
        reply_data: ctx.accounts.reply_data.to_account_info(),
        fee_handler: ctx.accounts.fee_handler.to_account_info(),
    };
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    let ticket = xcall::cpi::execute_call(cpi_context, _request_id, _data)?;

    let data: Vec<u8>= vec![];

    let (_, decoded_struct) = decode_execute_call_msg(&data)?;

    let res = match decoded_struct {
        ExecuteCallDecodedStruct::WithdrawNativeTo(data) => {
            // let cpi_accounts = Transfer {
        //     from: ctx.accounts.token_account.to_account_info(),
        //     to: ctx.accounts.token_account.to_account_info(),
        //     authority: ctx.accounts.user.to_account_info(),
        // };
        // let cpi_program = ctx.accounts.token_program.to_account_info();
        // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // token::transfer(cpi_ctx, amount)?;
        },
        ExecuteCallDecodedStruct::WithdrawTo(data) => {
            // let cpi_accounts = Transfer {
        //     from: ctx.accounts.token_account.to_account_info(),
        //     to: ctx.accounts.token_account.to_account_info(),
        //     authority: ctx.accounts.user.to_account_info(),
        // };
        // let cpi_program = ctx.accounts.token_program.to_account_info();
        // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // token::transfer(cpi_ctx, amount)?;
        },
    };

    Ok(())
}


fn send_deposit_message(
    config: &Config,
    token_address: String,
    from: Pubkey,
    amount: u64,
    to: Option<String>,
    data: Option<Vec<u8>>,
    fee: u64,
) {
    let deposit_message = DepositMessage::create(
        token_address.clone(),
        from.to_string(),
        to.unwrap_or("".to_string()),
        amount,
        data.unwrap_or(vec![]),
    );

    let data = deposit_message.encode();

    let deposit_revert = DepositRevert::create(token_address, from.to_string(), amount);

    let rollback = deposit_revert.encode();

    // let protocols = get_protocols();
}

fn calculate_limit(balance: u64, rate_limit: &RateLimit) -> u64 {
    let period = rate_limit.period;
    let percentage = rate_limit.percentage;
    if period == 0 {
        return 0;
    }

    let max_limit = (balance * percentage) / POINTS;
    let max_withdraw = balance - max_limit;
    let time_diff = Clock::get().unwrap().unix_timestamp as u64 - rate_limit.last_update;
    let time_diff = std::cmp::min(time_diff, period);

    let added_allowed_withdrawal = (max_withdraw * time_diff) / period;
    let limit = rate_limit.current_limit - added_allowed_withdrawal;
    std::cmp::min(balance, std::cmp::max(limit, max_limit))
}

// fn get_protocols(config: &Config) -> Protocols {
//     let cpi_program = config.xcall_manager.to_account_info();
//     let cpi_accounts = GetProtocols {};
//     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
//     let protocols = xcall_manager::cpi::get_protocols(cpi_ctx).get();
//     protocols
// }
