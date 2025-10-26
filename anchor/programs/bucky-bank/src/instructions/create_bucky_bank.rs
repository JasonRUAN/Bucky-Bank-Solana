use crate::{
    constants::*,
    error_code::BuckyBankError as ErrorCode,
    events::*,
    state::{BuckyBankInfo, BankGlobalStatsInfo, UserBuckyBanksInfo, BuckyBankStatus, Config},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateBuckyBank<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub bank_global_stats: Account<'info, BankGlobalStatsInfo>,

    #[account(
        init,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR_SIZE + BuckyBankInfo::INIT_SPACE,
        seeds = [bank_global_stats.total_bucky_banks.to_le_bytes().as_ref()],
        bump,
    )]
    pub bucky_bank: Account<'info, BuckyBankInfo>,

    #[account(
        init_if_needed,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR_SIZE + UserBuckyBanksInfo::INIT_SPACE,
        seeds = [USER_BUCKY_BANKS_SEED, owner.key().as_ref()],
        bump,
    )]
    pub user_bucky_banks: Account<'info, UserBuckyBanksInfo>,

    pub system_program: Program<'info, System>,
}

/// 创建存钱罐
pub fn _create_bucky_bank(
    ctx: Context<CreateBuckyBank>,
    name: String,
    target_amount: u64,
    duration_days: u64,
    child_address: Pubkey,
) -> Result<()> {
    // 验证输入
    require!(
        name.len() <= MAX_NAME_LENGTH,
        ErrorCode::InvalidName
    );
    require!(target_amount > 0, ErrorCode::InvalidAmount);
    require!(duration_days > 0, ErrorCode::InvalidDeadline);

    let current_time_ms = Clock::get()?.unix_timestamp as u64 * 1000;
    let deadline_ms = current_time_ms + (duration_days * 24 * 60 * 60 * 1000);

    let sender = ctx.accounts.owner.key();

    // 创建配置
    let config = Config {
        name,
        target_amount,
        deadline_ms,
        child_address,
    };

    // 初始化存钱罐
    let bucky_bank = &mut ctx.accounts.bucky_bank;
    bucky_bank.parent = sender;
    bucky_bank.config = config.clone();
    bucky_bank.current_balance = 0;
    bucky_bank.status = BuckyBankStatus::Active;
    bucky_bank.deposit_count = 0;
    bucky_bank.withdrawal_request_counter = 0;
    bucky_bank.created_at_ms = current_time_ms;
    bucky_bank.last_deposit_ms = current_time_ms;

    // 更新全局统计
    let stats = &mut ctx.accounts.bank_global_stats;
    stats.total_bucky_banks = stats.total_bucky_banks.checked_add(1)
        .ok_or(ErrorCode::Overflow)?;

    // 更新用户存钱罐列表
    let user_bucky_banks = &mut ctx.accounts.user_bucky_banks;
    if user_bucky_banks.owner == Pubkey::default() {
        user_bucky_banks.owner = sender;
    }
    user_bucky_banks.bucky_bank_ids.push(ctx.accounts.bucky_bank.key());

    // 发出事件
    emit!(BuckyBankCreated {
        bucky_bank_id: ctx.accounts.bucky_bank.key(),
        name: config.name,
        parent: sender,
        child: child_address,
        target_amount,
        created_at_ms: current_time_ms,
        deadline_ms,
        duration_days,
        current_balance: 0,
    });

    Ok(())
}
