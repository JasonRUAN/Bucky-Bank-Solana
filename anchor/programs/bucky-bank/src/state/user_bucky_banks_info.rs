use anchor_lang::prelude::*;

use crate::constants::MAX_BUCKY_BANK_ID_ARRAY_LENGTH;

#[account]
#[derive(InitSpace)]
pub struct UserBuckyBanksInfo {
    pub owner: Pubkey,

    #[max_len(MAX_BUCKY_BANK_ID_ARRAY_LENGTH)]
    pub bucky_bank_ids: Vec<Pubkey>,
}
