use crate::error_state::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
    TransferChecked,
};

#[derive(Accounts)]
#[instruction(label: String)]
pub struct EditTextData<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub creator: SystemAccount<'info>,

    #[account(
      mut,
      seeds = [b"permissions", creator.key().as_ref(), label.as_ref()],
      bump = encapsulated_data.bump,
      realloc = 8 + PermissionData::INIT_SPACE,
      realloc::payer = payer,
      realloc::zero = true
    )]
    pub encapsulated_data: Account<'info, PermissionData>,

    #[account(
        seeds = [b"permissioned_wallet".as_ref(), payer.key().as_ref(), label.as_ref()],
        bump = payer_permissioned_wallet.bump,
        constraint = payer_permissioned_wallet.main_data_pda == encapsulated_data.key()
            @ ErrorCode::Unauthorized
    )]
    pub payer_permissioned_wallet: Account<'info, PermissionedWallet>,

    pub system_program: Program<'info, System>,
}

impl<'info> EditTextData<'info> {
    pub fn edit_text_data(&mut self, _label: String, data: String) -> Result<()> {
        let encapsulated_data = &mut self.encapsulated_data;

        // Check if time limited and if so, check if the current time is between the start and end time
        if self.payer_permissioned_wallet.role == Role::TimeLimited {
            let current_time = Clock::get()?.unix_timestamp as u64;
            if current_time < self.payer_permissioned_wallet.start_time
                || current_time > self.payer_permissioned_wallet.end_time
            {
                return Err(error!(ErrorCode::Unauthorized));
            }
        }

        encapsulated_data.data = EncapsulatedData {
            text: Some(data),
            token: None,
        };
        Ok(())
    }
}

fn check_time_limited<'info>(
    payer_permissioned_wallet: &Account<'info, PermissionedWallet>,
) -> Result<()> {
    // Check if time limited and if so, check if the current time is between the start and end time
    if payer_permissioned_wallet.role == Role::TimeLimited {
        let current_time = Clock::get()?.unix_timestamp as u64;
        if current_time < payer_permissioned_wallet.start_time
            || current_time > payer_permissioned_wallet.end_time
        {
            return Err(error!(ErrorCode::Unauthorized));
        }
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(label: String)]
pub struct EditDepositTokenData<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub creator: SystemAccount<'info>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        has_one = token_mint,
        seeds = [b"escrow", creator.key().as_ref(), label.as_ref()],
        bump = escrow.bump
      )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = escrow,
      )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = payer,
    )]
    pub payer_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"permissioned_wallet".as_ref(), payer.key().as_ref(), label.as_ref()],
        bump = payer_permissioned_wallet.bump,
        constraint = payer_permissioned_wallet.main_data_pda == encapsulated_data.key()
            @ ErrorCode::Unauthorized
    )]
    pub payer_permissioned_wallet: Account<'info, PermissionedWallet>,

    #[account(
        mut,
        seeds = [b"permissions", creator.key().as_ref(), label.as_ref()],
        bump = encapsulated_data.bump,
        realloc = 8 + PermissionData::INIT_SPACE,
        realloc::payer = payer,
        realloc::zero = true
      )]
    pub encapsulated_data: Account<'info, PermissionData>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> EditDepositTokenData<'info> {
    pub fn edit_deposit_token_data(&mut self, label: String, amount: u64) -> Result<()> {
        check_time_limited(&self.payer_permissioned_wallet)?;

        let encapsulated_data = &mut self.encapsulated_data;

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.payer_ata.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.token_mint.to_account_info(),
            authority: self.payer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, self.token_mint.decimals)?;

        encapsulated_data.data = EncapsulatedData {
            text: None,

            token: Some(TokenData {
                token_mint: self.token_mint.key(),
                decimals: self.token_mint.decimals,
                token_amount: encapsulated_data
                    .data
                    .token
                    .as_ref()
                    .unwrap()
                    .token_amount
                    .checked_add(amount)
                    .ok_or(error!(ErrorCode::AmountTooLarge))?,
            }),
        };

        Ok(())
    }
}

fn close_escrow<'info>(
    vault: &InterfaceAccount<'info, TokenAccount>,
    owner: &SystemAccount<'info>,
    escrow: &Account<'info, Escrow>,
    creator: &SystemAccount<'info>,
    token_program: &Interface<'info, TokenInterface>,
    encapsulated_data: &mut Account<'info, PermissionData>,
    label: String,
) -> Result<()> {
    let cpi_program = token_program.to_account_info();

    let cpi_accounts = CloseAccount {
        account: vault.to_account_info(),
        destination: owner.to_account_info(),
        authority: escrow.to_account_info(),
    };

    let binding = creator.key();

    let seeds = &[b"escrow", binding.as_ref(), label.as_ref(), &[escrow.bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    close_account(cpi_ctx)?;

    empty_encapsulated_data(encapsulated_data)?;

    Ok(())
}

fn empty_encapsulated_data(encapsulated_data: &mut Account<'_, PermissionData>) -> Result<()> {
    encapsulated_data.data = EncapsulatedData {
        text: None,
        token: None,
    };
    Ok(())
}

#[derive(Accounts)]
#[instruction(label: String)]
pub struct EditTransferTokenData<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub creator: SystemAccount<'info>,

    pub taker: SystemAccount<'info>,

    pub owner: SystemAccount<'info>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        has_one = token_mint,
        seeds = [b"escrow", creator.key().as_ref(), label.as_ref()],
        bump = escrow.bump
      )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = escrow,
      )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = payer,
    )]
    pub payer_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = owner,
    )]
    pub owner_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"permissioned_wallet".as_ref(), payer.key().as_ref(), label.as_ref()],
        bump = payer_permissioned_wallet.bump,
        constraint = payer_permissioned_wallet.main_data_pda == encapsulated_data.key()
            @ ErrorCode::Unauthorized
    )]
    pub payer_permissioned_wallet: Account<'info, PermissionedWallet>,

    #[account(
        mut,
        seeds = [b"permissions", creator.key().as_ref(), label.as_ref()],
        bump = encapsulated_data.bump,
        realloc = 8 + PermissionData::INIT_SPACE,
        realloc::payer = payer,
        realloc::zero = true
      )]
    pub encapsulated_data: Account<'info, PermissionData>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> EditTransferTokenData<'info> {
    pub fn edit_transfer_token_data(&mut self, label: String, amount: u64) -> Result<()> {
        check_time_limited(&self.payer_permissioned_wallet)?;

        let encapsulated_data = &mut self.encapsulated_data;

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata.to_account_info(),
            mint: self.token_mint.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let binding = self.creator.key();
        let seeds = &[
            b"escrow",
            binding.as_ref(),
            label.as_ref(),
            &[self.escrow.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer_checked(cpi_ctx, amount, self.token_mint.decimals)?;

        if amount == encapsulated_data.data.token.as_ref().unwrap().token_amount {
            close_escrow(
                &self.vault,
                &self.owner,
                &self.escrow,
                &self.creator,
                &self.token_program,
                encapsulated_data,
                label,
            )?;
        } else {
            encapsulated_data.data = EncapsulatedData {
                text: None,
                token: Some(TokenData {
                    token_mint: self.token_mint.key(),
                    decimals: self.token_mint.decimals,
                    token_amount: encapsulated_data
                        .data
                        .token
                        .as_ref()
                        .unwrap()
                        .token_amount
                        .checked_sub(amount)
                        .ok_or(error!(ErrorCode::AmountTooLarge))?,
                }),
            };
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(label: String)]
pub struct EditWithdrawTokenData<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub creator: SystemAccount<'info>,

    pub owner: SystemAccount<'info>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        has_one = token_mint,
        seeds = [b"escrow", creator.key().as_ref(), label.as_ref()],
        bump = escrow.bump
      )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = escrow,
      )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = owner,
    )]
    pub owner_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"permissioned_wallet".as_ref(), payer.key().as_ref(), label.as_ref()],
        bump = payer_permissioned_wallet.bump,
        constraint = payer_permissioned_wallet.main_data_pda == encapsulated_data.key()
            @ ErrorCode::Unauthorized
    )]
    pub payer_permissioned_wallet: Account<'info, PermissionedWallet>,

    #[account(
        mut,
        seeds = [b"permissions", creator.key().as_ref(), label.as_ref()],
        bump = encapsulated_data.bump,
        realloc = 8 + PermissionData::INIT_SPACE,
        realloc::payer = payer,
        realloc::zero = true
      )]
    pub encapsulated_data: Account<'info, PermissionData>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> EditWithdrawTokenData<'info> {
    pub fn edit_withdraw_token_data(&mut self, label: String, amount: u64) -> Result<()> {
        check_time_limited(&self.payer_permissioned_wallet)?;

        let encapsulated_data = &mut self.encapsulated_data;

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.owner_ata.to_account_info(),
            mint: self.token_mint.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let binding = self.creator.key();
        let seeds = &[
            b"escrow",
            binding.as_ref(),
            label.as_ref(),
            &[self.escrow.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer_checked(cpi_ctx, amount, self.token_mint.decimals)?;

        if amount == encapsulated_data.data.token.as_ref().unwrap().token_amount {
            close_escrow(
                &self.vault,
                &self.owner,
                &self.escrow,
                &self.creator,
                &self.token_program,
                encapsulated_data,
                label,
            )?;
        } else {
            encapsulated_data.data = EncapsulatedData {
                text: None,
                token: Some(TokenData {
                    token_mint: self.token_mint.key(),
                    decimals: self.token_mint.decimals,
                    token_amount: encapsulated_data
                        .data
                        .token
                        .as_ref()
                        .unwrap()
                        .token_amount
                        .checked_sub(amount)
                        .ok_or(error!(ErrorCode::AmountTooLarge))?,
                }),
            };
        }

        Ok(())
    }
}
