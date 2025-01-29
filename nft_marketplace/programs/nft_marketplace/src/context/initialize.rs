use anchor_lang::prelude::*;

use anchor_spl::token_interface::{TokenInterface, Mint};

use crate::state::Marketplace;
use crate::error::MarketplaceError;

#[derive(Accounts)]
#[instruction(name: String)]

pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init, 
        payer = admin, 
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [b"marketplace", name.as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitializeMarketplace<'info> {
    pub fn initialize(&mut self, name: String, fee: u16, bumps: &InitializeMarketplaceBumps) -> Result<()> {

        require!(name.len() <= 32 && name.len() > 0, MarketplaceError::NameInvalid);

        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            reward_bump: bumps.reward_mint,
            name,
        });

        Ok(())
    }
}

