use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
#[instruction(label: String)]
pub struct InitEscrow<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = owner,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", crate::ID.as_ref(), label.as_ref()],
        bump
      )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = owner,
        associated_token::mint = token_mint,
        associated_token::authority = escrow,
      )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitEscrow<'info> {
    pub fn init_escrow(&mut self, _label: String, bumps: &InitEscrowBumps) -> Result<()> {
        self.escrow.set_inner(Escrow {
            owner: self.owner.key(),
            mint: self.token_mint.key(),
            bump: bumps.escrow,
        });

        Ok(())
    }
}
