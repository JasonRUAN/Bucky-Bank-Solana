use crate::{
    error_code::BuckyBankError as ErrorCode,
    events::*,
    state::{BuckyBankInfo, WithdrawalRequestInfo},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub bucky_bank: Account<'info, BuckyBankInfo>,

    #[account(mut, close = bucky_bank)]
    pub withdrawal_request: Account<'info, WithdrawalRequestInfo>,

    #[account(mut)]
    pub child: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// 小孩提取存款
pub fn _withdraw(ctx: Context<Withdraw>) -> Result<()> {
    let sender = ctx.accounts.child.key();
    let current_time_ms = Clock::get()?.unix_timestamp as u64 * 1000;

    let bucky_bank = &mut ctx.accounts.bucky_bank;
    let withdrawal_request = &mut ctx.accounts.withdrawal_request;

    // 验证权限和条件
    require!(
        sender == bucky_bank.config.child_address,
        ErrorCode::NotChild
    );

    require!(
        withdrawal_request.bucky_bank_id == bucky_bank.key(),
        ErrorCode::RequestNotFound
    );

    require!(
        withdrawal_request.approved_by == bucky_bank.parent,
        ErrorCode::NotParent
    );

    let amount = withdrawal_request.amount;

    // 验证金额有效性
    require!(amount > 0, ErrorCode::InvalidWithdrawalAmount);
    require!(
        amount <= bucky_bank.current_balance,
        ErrorCode::InsufficientFunds
    );

    // 更新存钱罐余额
    bucky_bank.current_balance = bucky_bank
        .current_balance
        .checked_sub(amount)
        .ok_or(ErrorCode::Overflow)?;

    **bucky_bank.to_account_info().try_borrow_mut_lamports()? -= amount;

    **ctx
        .accounts
        .child
        .to_account_info()
        .try_borrow_mut_lamports()? += amount;

    // 保存取款请求的 key 用于事件发送
    let request_id = withdrawal_request.key();

    // 发送事件
    emit!(EventWithdrawalCompleted {
        request_id,
        bucky_bank_id: bucky_bank.key(),
        amount,
        left_balance: bucky_bank.current_balance,
        withdrawer: sender,
        created_at_ms: current_time_ms,
    });

    Ok(())
}
