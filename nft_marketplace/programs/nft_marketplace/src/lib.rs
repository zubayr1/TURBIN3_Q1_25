use anchor_lang::prelude::*;

pub mod context;
pub mod error;
pub mod state;

pub use context::*;
pub use error::*;
pub use state::*;

declare_id!("9GNeaTdHRfYcFmN9LDqQCDro81BunE5Aftyk1dBbpwNh");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<InitializeMarketplace>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps)
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.initialize_listing(price, &ctx.bumps)
    }

    pub fn deposit_nft(ctx: Context<List>) -> Result<()> {
        ctx.accounts.deposit_nft()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()
    }

    pub fn send_sol(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol()
    }

    pub fn send_token(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_token()
    }

    pub fn close_vault(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.close_vault()
    }
}
