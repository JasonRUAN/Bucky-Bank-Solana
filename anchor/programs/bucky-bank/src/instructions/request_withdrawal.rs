use crate::{
    constants::*,
    error_code::BuckyBankError as ErrorCode,
    events::*,
    state::{BuckyBankInfo, WithdrawalRequestInfo, WithdrawalStatus},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RequestWithdrawal<'info> {
    #[account(mut)]
    pub bucky_bank: Account<'info, BuckyBankInfo>,

    #[account(
        init,
        payer = requester,
        space = ANCHOR_DISCRIMINATOR_SIZE + WithdrawalRequestInfo::INIT_SPACE,
        seeds = [
            WITHDRAWAL_REQUEST_SEED,
            bucky_bank.key().as_ref(),
            requester.key().as_ref(),
        ],
        bump,
    )]
    pub withdrawal_request: Account<'info, WithdrawalRequestInfo>,

    #[account(mut)]
    pub requester: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// 取款请求功能 - 只有孩子可以请求取款
pub fn _request_withdrawal(
    ctx: Context<RequestWithdrawal>,
    amount: u64,
    reason: String,
) -> Result<()> {
    let sender = ctx.accounts.requester.key();
    let current_time_ms = Clock::get()?.unix_timestamp as u64 * 1000;

    let bucky_bank = &ctx.accounts.bucky_bank;

    // 验证权限和状态
    require!(
        sender == bucky_bank.config.child_address,
        ErrorCode::NotChildForWithdrawal
    );

    require!(
        amount > 0 && amount <= bucky_bank.current_balance,
        ErrorCode::InvalidWithdrawalAmount
    );

    require!(
        reason.len() <= MAX_REASON_LENGTH,
        ErrorCode::ReasonTooLong
    );

    // 创建取款请求
    let withdrawal_request = &mut ctx.accounts.withdrawal_request;
    withdrawal_request.bucky_bank_id = bucky_bank.key();
    withdrawal_request.requester = sender;
    withdrawal_request.amount = amount;
    withdrawal_request.reason = reason.clone();
    withdrawal_request.status = WithdrawalStatus::Pending;
    withdrawal_request.approved_by = bucky_bank.parent;
    withdrawal_request.created_at_ms = current_time_ms;
    withdrawal_request.approved_at_ms = 0;

    // 发送事件
    emit!(EventWithdrawalRequested {
        request_id: ctx.accounts.withdrawal_request.key(),
        bucky_bank_id: bucky_bank.key(),
        amount,
        requester: sender,
        reason,
        status: WithdrawalStatus::Pending as u8,
        approved_by: bucky_bank.parent,
        created_at_ms: current_time_ms,
    });

    Ok(())
}
