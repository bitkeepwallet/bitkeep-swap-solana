use anchor_lang::prelude::*;

#[account]
pub struct Bkswap {
    pub authority: Pubkey,
    pub fee_receiver: Pubkey,
    pub fee_rate: u16,
}
