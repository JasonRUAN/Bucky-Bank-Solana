use crate::{
    constants::*,
    error_code::BuckyBankError as ErrorCode,
    events::*,
    state::{BuckyBankInfo, BankGlobalStatsInfo, BuckyBankStatus},
};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [BANK_GLOBAL_STATS_SEED],
        bump,
    )]
    pub bank_global_stats: Account<'info, BankGlobalStatsInfo>,

    #[account(mut)]
    pub bucky_bank: Account<'info, BuckyBankInfo>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// 存款功能 - 只有孩子可以存款
pub fn _deposit(
    ctx: Context<Deposit>,
    deposit_balance: u64, // lamports
) -> Result<()> {
    let sender = ctx.accounts.depositor.key();
    let current_time_ms = Clock::get()?.unix_timestamp as u64 * 1000;

    let bucky_bank = &mut ctx.accounts.bucky_bank;

    // 验证权限和状态
    require!(
        bucky_bank.status == BuckyBankStatus::Active,
        ErrorCode::BankNotActive
    );
    require!(
        sender == bucky_bank.config.child_address,
        ErrorCode::NotChild
    );

    require!(
        deposit_balance >= 10_000_000,
        ErrorCode::InvalidDepositAmount
    ); // 存款金额要 >=0.01 SOL

    let transfer_instruction = system_instruction::transfer(
        &ctx.accounts.depositor.key(), // 转出账户
        &bucky_bank.key(), // 转入账户
        deposit_balance, // 转账金额
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            ctx.accounts.depositor.to_account_info(), // to_account_info 用来获取账户信息
            bucky_bank.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // 更新存钱罐余额
    bucky_bank.current_balance = bucky_bank.current_balance
        .checked_add(deposit_balance)
        .ok_or(ErrorCode::Overflow)?;

    // 更新统计
    bucky_bank.deposit_count = bucky_bank.deposit_count
        .checked_add(1)
        .ok_or(ErrorCode::Overflow)?;
    bucky_bank.last_deposit_ms = current_time_ms;

    // 检查是否达到目标
    if bucky_bank.current_balance >= bucky_bank.config.target_amount {
        bucky_bank.status = BuckyBankStatus::Completed;
    }

    // 发送事件
    emit!(DepositMade {
        bucky_bank_id: ctx.accounts.bucky_bank.key(),
        amount: deposit_balance,
        depositor: sender,
        created_at_ms: current_time_ms,
    });

    Ok(())
}
