use anchor_lang::prelude::*;

mod instructions;
mod states;

use instructions::*;
use states::*;

declare_id!("Hd1G3Newy87gKKbDMx9sHSYH9FnGUewKxxFeEVmxpPxy");

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<InitializeMarketplace>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps)
    }

    pub fn create_listing(ctx: Context<CreateListing>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)
    }

    pub fn deposit(ctx: Context<CreateListing>) -> Result<()> {
        ctx.accounts.deposit()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist()
    }

    pub fn send_token(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_token()
    }

    pub fn close(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.close()
    }
}
