use anchor_lang::prelude::*;

pub mod states;
pub mod instructions;
pub mod message_types;
pub mod error;

use states::*;

use anchor_spl::{
    associated_token::AssociatedToken, metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, 
        Metadata as Metaplex,
    }, token::{mint_to, Mint, MintTo,transfer, Transfer, Token, TokenAccount, burn, Burn}
};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod balanced_dollar {
    use anchor_spl::token;
    use error::BalancedDollarError;
    use message_types::{CrossTransferMessage, CrossTransferRevert, NetworkAddress, CROSS_TRANSFER, CROSS_TRANSFER_REVERT};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, xcall_address: Pubkey, icon_bnusd: String, xcall_manager: Pubkey) -> Result<()> {
        let seeds = &["mint".as_bytes(), &[ctx.bumps.mint]];
        let signer = [&seeds[..]];

        let token_data: DataV2 = DataV2 {
            name: "Balanced Dollar".to_string(),
            symbol: "bnUSD".to_string(),
            uri: "https://raw.githubusercontent.com/balancednetwork/assets/master/blockchains/icon/assets/cx88fd7df7ddff82f7cc735c871dc519838cb235bb/logo.png".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let metadata_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.mint.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
                mint_authority: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &signer
        );

        create_metadata_accounts_v3(
            metadata_ctx,
            token_data,
            false,
            true,
            None,
        )?;

        let balanced_dollar = &mut ctx.accounts.balanced_dollar;
        balanced_dollar.xcall = xcall_address;
        balanced_dollar.icon_bnusd = icon_bnusd;
        balanced_dollar.xcall_manager = xcall_manager;
        balanced_dollar.mint_authority = ctx.accounts.payer.key();
        balanced_dollar.total_supply = 0;
        Ok(())
    }


    pub fn cross_transfer(
        ctx: Context<CrossTransfer>,
        amount: u64,
        to: String,
        data: Option<Vec<u8>>,
    ) -> Result<()> {
        require!(amount > 0, BalancedDollarError::InvalidAmount);

        let token_account = &ctx.accounts.token;
        let token_account_info = token_account.to_account_info();
        let token_account_data = token::accessor::amount(&token_account_info)?;
        require!(token_account_data >= amount, BalancedDollarError::InvalidAmount);

        let cpi_accounts = Transfer {
            from: ctx.accounts.token.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        let cpi_accounts_burn = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        let cpi_ctx_burn = CpiContext::new(cpi_program, cpi_accounts_burn);
        token::burn(cpi_ctx_burn, amount)?;

        // Construct message data
        let from_address = *ctx.accounts.sender.key;
        let message_data = data.unwrap_or_default();
        let xcall_message_struct = CrossTransferMessage {
            method: CROSS_TRANSFER.to_string(),
            from: NetworkAddress::from_str(&from_address.to_string().clone()),
            to: NetworkAddress::from_str(&to),
            value: amount.into(),
            data: message_data,
        };
        let rollback_struct = CrossTransferRevert{
            method: CROSS_TRANSFER_REVERT.to_string(),
            from: from_address,
            value: amount.into(),
        };

        Ok(())
    }


}
