#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod error_state;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod one_cs {
    use super::*;

    /// Encapsulate the data and create a new permission data account
    pub fn encapsulate(ctx: Context<Encapsulate>, label: String, data: String) -> Result<()> {
        ctx.accounts.encapsulate(&ctx.bumps, label, data)?;

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
            .add_permission(_label, role_index, start_time, end_time)?;

        Ok(())
    }

    /// Remove a permission from the permission list
    pub fn remove_permission(ctx: Context<RemovePermission>, _label: String) -> Result<()> {
        ctx.accounts.remove_permission(_label)?;

        Ok(())
    }

    /// Edit the encapsulated data
    pub fn edit_data(ctx: Context<EditData>, _label: String, data: String) -> Result<()> {
        ctx.accounts.edit_data(_label, data)?;

        Ok(())
    }

    /// Transfer ownership of the permission data account
    pub fn transfer_ownership(
        ctx: Context<TransferOwnership>,
        _label: String,
        new_owner: Pubkey,
    ) -> Result<()> {
        ctx.accounts.transfer_ownership(_label, new_owner)?;

        Ok(())
    }
}
