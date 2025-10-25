use anchor_lang::prelude::*;

#[error_code]
pub enum BuckyBankError {
    #[msg("Deposit amount must be at least 0.01 SOL")]
    DepositTooSmall,
    #[msg("Insufficient funds for withdrawal")]
    InsufficientFunds,
    #[msg("存钱罐名称无效")]
    InvalidName,
    #[msg("目标金额无效")]
    InvalidAmount,
    #[msg("截止日期无效")]
    InvalidDeadline,
    #[msg("数值溢出")]
    Overflow,
    #[msg("存钱罐不处于活跃状态")]
    BankNotActive,
    #[msg("只有孩子可以存款")]
    NotChild,
    #[msg("存款金额无效")]
    InvalidDepositAmount,
    #[msg("只有孩子可以请求取款")]
    NotChildForWithdrawal,
    #[msg("取款金额无效")]
    InvalidWithdrawalAmount,
    #[msg("取款原因过长")]
    ReasonTooLong,
    #[msg("只有家长可以审批取款请求")]
    NotParent,
    #[msg("取款请求状态无效")]
    InvalidRequestStatus,
    #[msg("取款请求不存在")]
    RequestNotFound,
}