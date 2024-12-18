use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event {
    // data
    #[max_len(16)] // event name should be 16 characters or less
    pub id: String,
    #[max_len(40)] // event name should be 40 characters or less
    pub name: String,
    #[max_len(150)] // event description should be 150 characters or less
    pub description: String,
    // prices
    pub ticket_price: f64,
    pub token_price: f64,
    // event status
    pub active: bool,
    pub total_sponsors: u64, // event all time sponsors
    pub current_sponsors: u64, // event current sponsors
    pub tokens_sold: u64, // event sponsor tokens sold
    pub tickets_sold: u64, // event tickets sold
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