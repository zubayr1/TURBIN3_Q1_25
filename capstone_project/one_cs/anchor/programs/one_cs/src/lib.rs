#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod error_state;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("CMKPvVkdwcFBhKNg47jbV3hwrGDQFu8rSsWGh7M12utA");

#[program]
pub mod one_cs {
    use super::*;

    /// Encapsulate the text data and create a new permission data account
    pub fn encapsulate_text(
        ctx: Context<EncapsulateText>,
        label: String,
        data: String,
    ) -> Result<()> {
        ctx.accounts.encapsulate_text(&ctx.bumps, label, data)?;

        Ok(())
    }

    /// Add a new permission to the permission list
    pub fn add_permission(
        ctx: Context<AddPermission>,
        _label: String,
        role_index: u64,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        ctx.accounts
            .add_permission(_label, role_index, start_time, end_time, &ctx.bumps)?;

        Ok(())
    }

    /// Remove a permission from the permission list
    pub fn remove_permission(ctx: Context<RemovePermission>, _label: String) -> Result<()> {
        ctx.accounts.remove_permission(_label)?;

        Ok(())
    }

    /// Edit the encapsulated text data
    pub fn edit_text_data(ctx: Context<EditTextData>, _label: String, data: String) -> Result<()> {
        ctx.accounts.edit_text_data(_label, data)?;

        Ok(())
    }

    /// Transfer ownership of the permission data account
    pub fn transfer_ownership(
        ctx: Context<TransferOwnership>,
        _label: String,
        ownership_time: u64,
    ) -> Result<()> {
        ctx.accounts
            .transfer_ownership(_label, ownership_time, &ctx.bumps)?;

        Ok(())
    }

    /// Accept ownership of the permission data account
    pub fn accept_ownership(ctx: Context<AcceptOwnership>, _label: String) -> Result<()> {
        ctx.accounts.accept_ownership(_label, &ctx.bumps)?;

        Ok(())
    }

    /// Init escrow
    pub fn init_escrow(ctx: Context<InitEscrow>, label: String) -> Result<()> {
        ctx.accounts.init_escrow(label, &ctx.bumps)?;

        Ok(())
    }

    /// Deposit tokens into the escrow
    pub fn deposit_tokens(ctx: Context<DepositEscrow>, label: String, amount: u64) -> Result<()> {
        ctx.accounts.deposit_tokens(label, amount)?;

        Ok(())
    }

    /// Encapsulate a token
    pub fn encapsulate_token(ctx: Context<EncapsulateToken>, label: String) -> Result<()> {
        ctx.accounts.encapsulate_token(label, &ctx.bumps)?;

        Ok(())
    }

    /// Edit the encapsulated token data: Deposit
    pub fn edit_deposit_token_data(
        ctx: Context<EditDepositTokenData>,
        label: String,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.edit_deposit_token_data(label, amount)?;

        Ok(())
    }

    /// Edit the encapsulated token data: Transfer  
    pub fn edit_transfer_token_data(
        ctx: Context<EditTransferTokenData>,
        label: String,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.edit_transfer_token_data(label, amount)?;

        Ok(())
    }

    /// Edit the encapsulated token data: Withdraw
    pub fn edit_withdraw_token_data(
        ctx: Context<EditWithdrawTokenData>,
        label: String,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.edit_withdraw_token_data(label, amount)?;

        Ok(())
    }

    /// Close the escrow manually
    pub fn close_escrow_manually(ctx: Context<CloseEscrowManually>, label: String) -> Result<()> {
        ctx.accounts.close_escrow_manually(label)?;

        Ok(())
    }
}
