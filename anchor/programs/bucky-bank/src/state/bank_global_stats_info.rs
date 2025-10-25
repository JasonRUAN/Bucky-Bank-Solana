use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BankGlobalStatsInfo {
    pub total_bucky_banks: u64, // 存钱罐数量
    pub total_deposits: u64,  // 存款数量
    pub total_withdrawals: u64, // 取款数量
    pub admin: Pubkey,
}