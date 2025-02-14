use crate::error_state::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(label: String)]
pub struct AddPermission<'info> {
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

    pub permitted_wallet: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddPermission<'info> {
    pub fn add_permission(
        &mut self,
        _label: String,
        role_index: u64,
        payer_index: u64,
        permitted_wallet_index: i64,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        let encapsulated_data = &mut self.encapsulated_data;

        let role = match role_index {
            2 => Role::Admin,
            3 => Role::FullAccess,
            4 => Role::TimeLimited,
            _ => return Err(error!(ErrorCode::BadRole)),
        };

        // Check overflow
        if payer_index >= encapsulated_data.permissions.len() as u64 {
            return Err(error!(ErrorCode::Unauthorized));
        }

        // Check overflow
        if permitted_wallet_index >= encapsulated_data.permissions.len() as i64 {
            return Err(error!(ErrorCode::Unauthorized));
        }

        // Check if the payer exists. If not, return an error
        if encapsulated_data.permissions[payer_index as usize].wallet != self.payer.key() {
            return Err(error!(ErrorCode::Unauthorized));
        }

        // Check if the payer has the full access or time limited access. If so, return an error.
        if encapsulated_data.permissions[payer_index as usize].role == Role::FullAccess
            || encapsulated_data.permissions[payer_index as usize].role == Role::TimeLimited
        {
            return Err(error!(ErrorCode::Unauthorized));
        }

        let permitted_wallet = self.permitted_wallet.key();

        // Check if the permitted wallet exists if index is not -1. If not, return an error
        if permitted_wallet_index >= 0 {
            if permitted_wallet
                != encapsulated_data.permissions[permitted_wallet_index as usize].wallet
            {
                return Err(error!(ErrorCode::Unauthorized));
            }
        }

        // If payer is admin, payer can't demote another admin to full access or time limited
        // Else (providing wallet already exists), update the permission
        // Else (providing wallet doesn't exist), add the permission
        if permitted_wallet_index >= 0 {
            if encapsulated_data.permissions[payer_index as usize].role == Role::Admin
                && encapsulated_data.permissions[permitted_wallet_index as usize].role
                    == Role::Admin
                && (role == Role::FullAccess || role == Role::TimeLimited)
            {
                return Err(error!(ErrorCode::Unauthorized));
            }

            if role == Role::TimeLimited {
                // extra check for time limited permission
                if start_time >= end_time {
                    return Err(ErrorCode::InvalidTime.into());
                } else {
                    encapsulated_data.permissions[permitted_wallet_index as usize].start_time =
                        start_time;
                    encapsulated_data.permissions[permitted_wallet_index as usize].end_time =
                        end_time;
                }
            } else {
                encapsulated_data.permissions[permitted_wallet_index as usize].start_time = 0;
                encapsulated_data.permissions[permitted_wallet_index as usize].end_time = 0;
            }

            encapsulated_data.permissions[permitted_wallet_index as usize].role = role;
        } else {
            if role == Role::TimeLimited {
                // extra check for time limited permission
                if start_time >= end_time {
                    return Err(ErrorCode::InvalidTime.into());
                } else {
                    encapsulated_data.permissions.push(Permission {
                        role: role,
                        wallet: permitted_wallet,
                        start_time: start_time,
                        end_time: end_time,
                    });
                }
            } else {
                encapsulated_data.permissions.push(Permission {
                    role: role,
                    wallet: permitted_wallet,
                    start_time: 0,
                    end_time: 0,
                });
            }
        }

        Ok(())
    }
}
