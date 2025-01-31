use anchor_lang::prelude::*;

mod contexts;
mod errors;
mod states;

use contexts::*;
use errors::*;
use states::*;

declare_id!("HwsFCHU2tpzztZXi4guTxZgizcNPMfaVM7PDX1RHrtte");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }
}
