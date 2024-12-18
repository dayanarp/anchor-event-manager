use {crate::collections::Event, anchor_lang::prelude::*, anchor_spl::token::*};

#[derive(Accounts)]
#[instruction(id: String)]
pub struct CreateEvent<'info> {
    #[account(
        init,
        seeds = [
            id.to_string().as_ref(),
            Event::SEED_EVENT.as_bytes(), // "event"
            authority.key().as_ref(), // event authority
        ],
        bump,
        payer = authority,
        space = 8 + Event::INIT_SPACE
    )]
    pub event: Box<Account<'info, Event>>, // event account

    pub accepted_mint: Box<Account<'info, Mint>>, // accepted mint

    #[account(
        init,
        seeds = [
            Event::SEED_EVENT_MINT.as_bytes(), // "event_mint"
            event.key().as_ref() // event public key
        ],
        bump,
        payer = authority, 
        mint::decimals = 0, // no decimals
        mint::authority = event, 
    )]
    pub event_mint: Box<Account<'info, Mint>>, // event token mint

    #[account(
        init,
        payer = authority,
        seeds = [
            Event::SEED_TREASURY_VAULT.as_bytes(),  // "treasury_vault"
            event.key().as_ref() // event public key
        ],
        bump,
        token::mint = accepted_mint, // accepted mint
        token::authority = event,
    )]
    pub treasury_vault: Box<Account<'info, TokenAccount>>, // event treasury vault

    #[account(
        init,
        payer = authority,
        seeds = [
            Event::SEED_GAIN_VAULT.as_bytes(), // "gain_vault"
            event.key().as_ref() // event public key
        ],
        bump,
        token::mint = accepted_mint, // accepted mint
        token::authority = event,
    )]
    pub gain_vault: Box<Account<'info, TokenAccount>>, // event gain vault

    #[account(mut)]
    pub authority: Signer<'info>, // event authority
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<CreateEvent>,
    id: String,
    name: String,
    description: String,
    ticket_price: f64,
    token_price: f64
) -> Result<()> {
    // data
    ctx.accounts.event.id = id;
    ctx.accounts.event.name = name;
    ctx.accounts.event.description = description;
    // prices
    ctx.accounts.event.ticket_price = ticket_price;
    ctx.accounts.event.token_price = token_price;
    // event status
    ctx.accounts.event.active = true;
    ctx.accounts.event.total_sponsors = 0;
    ctx.accounts.event.current_sponsors = 0;
    ctx.accounts.event.tickets_sold = 0;
    ctx.accounts.event.tokens_sold = 0;
    // accounts
    ctx.accounts.event.authority = ctx.accounts.authority.key();
    ctx.accounts.event.accepted_mint = ctx.accounts.accepted_mint.key();
    // bumps
    ctx.accounts.event.event_bump = ctx.bumps.event;
    ctx.accounts.event.event_mint_bump = ctx.bumps.event_mint;
    ctx.accounts.event.treasury_vault_bump = ctx.bumps.treasury_vault;
    ctx.accounts.event.gain_vault_bump = ctx.bumps.gain_vault;
    Ok(())
}