use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

#[event]
pub struct BuckyBankCreated {
    pub bucky_bank_id: Pubkey,
    pub name: String,
    pub parent: Pubkey,
    pub child: Pubkey,
    pub target_amount: u64,
    pub created_at_ms: u64,
    pub deadline_ms: u64,
    pub duration_days: u64,
    pub current_balance: u64,
}

#[event]
pub struct DepositMade {
    pub bucky_bank_id: Pubkey,
    pub amount: u64,
    pub depositor: Pubkey,
    pub created_at_ms: u64,
}

#[event]
pub struct EventWithdrawalRequested {
    pub request_id: Pubkey,
    pub bucky_bank_id: Pubkey,
    pub amount: u64,
    pub requester: Pubkey,
    pub reason: String,
    pub status: u8, // 0-Pending, 1-Approved, 2-Rejected, 3-Completed
    pub approved_by: Pubkey,
    pub created_at_ms: u64,
}

#[event]
pub struct EventWithdrawalApproved {
    pub request_id: Pubkey,
    pub bucky_bank_id: Pubkey,
    pub amount: u64,
    pub approved_by: Pubkey,
    pub requester: Pubkey,
    pub reason: String,
    pub created_at_ms: u64,
}

#[event]
pub struct EventWithdrawalRejected {
    pub request_id: Pubkey,
    pub bucky_bank_id: Pubkey,
    pub amount: u64,
    pub requester: Pubkey,
    pub rejected_by: Pubkey,
    pub reason: String,
    pub created_at_ms: u64,
}

#[event]
pub struct EventWithdrawalCompleted {
    pub request_id: Pubkey,
    pub bucky_bank_id: Pubkey,
    pub amount: u64,
    pub left_balance: u64,
    pub withdrawer: Pubkey,
    pub created_at_ms: u64,
}
