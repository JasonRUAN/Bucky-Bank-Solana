use anchor_lang::prelude::*;

use crate::constants::*;

/// 存钱罐配置
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
#[derive(InitSpace)]
pub struct Config {
    /// 存钱罐名称
    #[max_len(MAX_NAME_LENGTH)]
    pub name: String,
    /// 目标存款金额（单位：lamports）
    pub target_amount: u64,
    /// 存款截止时间戳（毫秒）
    pub deadline_ms: u64,
    /// 存钱罐使用方地址
    pub child_address: Pubkey,
}

/// 存钱罐状态枚举
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
#[derive(InitSpace)]
pub enum BuckyBankStatus {
    /// 进行中
    Active = 0,
    /// 已完成
    Completed = 1,
    /// 已失败
    Failed = 2,
}

/// 存钱罐对象
#[account]
#[derive(InitSpace)]
pub struct BuckyBankInfo {
    /// 存钱罐创建方地址
    pub parent: Pubkey,
    /// 存钱罐配置
    pub config: Config,
    /// 当前存款余额（单位：lamports）
    pub current_balance: u64,
    /// 存钱罐状态
    pub status: BuckyBankStatus, // 0-进行中、1-已完成、2-失败
    /// 存款次数
    pub deposit_count: u64,
    /// 存钱罐创建时间（毫秒）
    pub created_at_ms: u64,
    /// 最近一次存款时间（毫秒）
    pub last_deposit_ms: u64,
}