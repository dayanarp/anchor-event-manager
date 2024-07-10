use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event {
    // data
    #[max_len(40)] // event name should be 40 characters or less
    pub name: String,
    pub ticket_price: u64,
    pub active: bool,
    pub sponsors: u64,
    // accounts
    pub authority: Pubkey,
    pub accepted_mint: Pubkey,
    // bumps
    pub event_bump: u8,
    pub event_mint_bump: u8,
    pub treasury_vault_bump: u8,
    pub gain_vault_bump: u8,
}

impl Event {
    // custom seeds
    pub const SEED_EVENT: &'static str = "event";
    pub const SEED_EVENT_MINT: &'static str = "event_mint";
    pub const SEED_TREASURY_VAULT: &'static str = "treasury_vault";
    pub const SEED_GAIN_VAULT: &'static str = "gain_vault";
}