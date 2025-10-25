use crate::{
    error_code::BuckyBankError as ErrorCode,
    events::*,
    state::{BuckyBankInfo, WithdrawalRequestInfo, WithdrawalStatus},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ApproveWithdrawal<'info> {
    #[account(mut)]
    pub bucky_bank: Account<'info, BuckyBankInfo>,

    #[account(mut)]
    pub withdrawal_request: Account<'info, WithdrawalRequestInfo>,

    #[account(mut)]
    pub parent: Signer<'info>,
}

/// 家长审批取款请求
pub fn _approve_withdrawal(
    ctx: Context<ApproveWithdrawal>,
    approve: bool,
    reason: String,
) -> Result<()> {
    let sender = ctx.accounts.parent.key();
    let current_time_ms = Clock::get()?.unix_timestamp as u64 * 1000;

    let bucky_bank = &mut ctx.accounts.bucky_bank;
    let withdrawal_request = &mut ctx.accounts.withdrawal_request;

    // 验证权限和状态
    require!(sender == bucky_bank.parent, ErrorCode::NotParent);

    require!(
        withdrawal_request.status == WithdrawalStatus::Pending,
        ErrorCode::InvalidRequestStatus
    );

    require!(
        withdrawal_request.bucky_bank_id == bucky_bank.key(),
        ErrorCode::RequestNotFound
    );

    let request_id = withdrawal_request.key();
    let amount = withdrawal_request.amount;
    let requester = withdrawal_request.requester;

    if approve {
        // 检查余额是否足够提取
        require!(
            amount <= bucky_bank.current_balance,
            ErrorCode::InsufficientFunds
        );

        // 更新状态为已授权
        withdrawal_request.status = WithdrawalStatus::Approved;
        withdrawal_request.approved_by = sender;
        withdrawal_request.approved_at_ms = current_time_ms;

        // 发送事件
        emit!(EventWithdrawalApproved {
            request_id,
            bucky_bank_id: bucky_bank.key(),
            amount,
            approved_by: sender,
            requester,
            reason: reason.clone(),
            created_at_ms: current_time_ms,
        });
    } else {
        // 拒绝请求
        withdrawal_request.status = WithdrawalStatus::Rejected;
        withdrawal_request.approved_by = sender;
        withdrawal_request.approved_at_ms = current_time_ms;

        // 发送事件
        emit!(EventWithdrawalRejected {
            request_id,
            bucky_bank_id: bucky_bank.key(),
            amount,
            requester,
            rejected_by: sender,
            reason: reason.clone(),
            created_at_ms: current_time_ms,
        });
    };

    Ok(())
}
