use anchor_lang::prelude::*;

use crate::constants::*;

/// 取款请求状态枚举
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
#[derive(InitSpace)]
pub enum WithdrawalStatus {
    /// 待审批
    Pending = 0,
    /// 已批准
    Approved = 1,
    /// 已拒绝
    Rejected = 2,
    /// 已完成
    Completed = 3,
}

/// 取款请求对象
#[account]
#[derive(InitSpace)]
pub struct WithdrawalRequestInfo {
    /// 存钱罐 ID
    pub bucky_bank_id: Pubkey,
    /// 请求者地址（孩子）
    pub requester: Pubkey,
    /// 取款金额（单位：lamports）
    pub amount: u64,
    /// 取款原因
    #[max_len(MAX_REASON_LENGTH)]
    pub reason: String,
    /// 取款请求状态
    pub status: WithdrawalStatus,
    /// 审批者地址（家长）
    pub approved_by: Pubkey,
    /// 创建时间（毫秒）
    pub created_at_ms: u64,
    /// 审批时间（毫秒）
    pub approved_at_ms: u64,
}
