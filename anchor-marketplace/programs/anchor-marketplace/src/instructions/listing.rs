use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::states::{Listing, Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        seeds = [b"marketplace".as_ref(), maker.key().as_ref(), name.as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub maker_mint: InterfaceAccount<'info, Mint>,

    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = 8 + Listing::INIT_SPACE,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"metadata".as_ref(), metadata_program.key().as_ref(), maker_mint.key().as_ref()],
        bump,
        seeds::program = metadata_program.key(),
        constraint = metadata_account.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata_account.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata_account: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"metadata".as_ref(), metadata_program.key().as_ref(), maker_mint.key().as_ref(), b"edition".as_ref()],
        bump,
        seeds::program = metadata_program.key(),
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> CreateListing<'info> {
    pub fn create_listing(&mut self, price: u64, bumps: &CreateListingBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            maker: self.maker.key(),
            mint: self.maker_mint.key(),
            price,
            bump: bumps.listing,
        });

        Ok(())
    }

    pub fn deposit(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.maker_ata.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;

        Ok(())
    }
}
