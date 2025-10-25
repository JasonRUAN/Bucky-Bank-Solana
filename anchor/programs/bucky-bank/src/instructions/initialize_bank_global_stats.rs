use crate::{
    constants::{ANCHOR_DISCRIMINATOR_SIZE, BANK_GLOBAL_STATS_SEED},
    state::BankGlobalStatsInfo,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeBankGlobalStats<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR_SIZE + BankGlobalStatsInfo::INIT_SPACE,
        seeds = [BANK_GLOBAL_STATS_SEED],
        bump,
    )]
    pub bank_global_stats: Account<'info, BankGlobalStatsInfo>,

    pub system_program: Program<'info, System>,
}

pub fn _initialize_bank_global_stats(ctx: Context<InitializeBankGlobalStats>) -> Result<()> {
    let bank_global_stats= &mut ctx.accounts.bank_global_stats;
    bank_global_stats.admin = ctx.accounts.owner.key();
    bank_global_stats.total_bucky_banks = 0;
    bank_global_stats.total_deposits = 0;
    bank_global_stats.total_withdrawals = 0;

    Ok(())
}
