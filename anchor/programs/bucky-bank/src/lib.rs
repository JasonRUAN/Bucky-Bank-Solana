#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;

pub mod constants;
pub mod error_code;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::initialize_bank_global_stats::*;
use instructions::create_bucky_bank::*;
use instructions::deposit::*;
use instructions::request_withdrawal::*;
use instructions::approve_withdrawal::*;
use instructions::withdraw::*;

declare_id!("5uykfXh94mwTz2Vxb2Y7RpntvanSkcWq4hENoDM4duVf");

#[program]
pub mod bucky_bank {
    use super::*;

    pub fn initialize_bank_global_stats(ctx: Context<InitializeBankGlobalStats>) -> Result<()> {
        _initialize_bank_global_stats(ctx)
    }

    pub fn create_bucky_bank(
        ctx: Context<CreateBuckyBank>,
        name: String,
        target_amount: u64,
        duration_days: u64,
        child_address: Pubkey,
    ) -> Result<()> {
        _create_bucky_bank(ctx, name, target_amount, duration_days, child_address)
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        deposit_balance: u64,
    ) -> Result<()> {
        _deposit(ctx, deposit_balance)
    }

    pub fn request_withdrawal(
        ctx: Context<RequestWithdrawal>,
        amount: u64,
        reason: String,
    ) -> Result<()> {
        _request_withdrawal(ctx, amount, reason)
    }

    pub fn approve_withdrawal(
        ctx: Context<ApproveWithdrawal>,
        approve: bool,
        reason: String,
    ) -> Result<()> {
        _approve_withdrawal(ctx, approve, reason)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        _withdraw(ctx)
    }
}
