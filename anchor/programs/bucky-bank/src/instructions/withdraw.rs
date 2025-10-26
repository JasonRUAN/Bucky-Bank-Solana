use crate::{
    error_code::BuckyBankError as ErrorCode,
    events::*,
    state::{BuckyBankInfo, WithdrawalRequestInfo, WithdrawalStatus},
};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

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

/// 小孩提取存款（若审批通过）
/// 
/// 此指令允许孩子在取款请求被家长批准后提取资金。
/// 
/// # 参数
/// 无额外参数
/// 
/// # 验证
/// - 调用者必须是孩子地址
/// - 取款请求必须属于该存钱罐
/// - 取款请求必须已被批准
/// - 存钱罐余额必须足够
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
    bucky_bank.current_balance = bucky_bank.current_balance
        .checked_sub(amount)
        .ok_or(ErrorCode::Overflow)?;

    **bucky_bank
        .to_account_info()
        .try_borrow_mut_lamports()? -= amount;

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
