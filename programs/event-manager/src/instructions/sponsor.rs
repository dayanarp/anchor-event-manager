use {
    crate::collections::Event, 
    anchor_lang::prelude::*, 
    anchor_spl::{
        token::*,
        associated_token::*,
    },
};

#[derive(Accounts)]
pub struct Sponsor<'info> {
    #[account(mut)]
    pub event: Box<Account<'info, Event>>, // event account

    #[account(
      mut,
      seeds = [
        Event::SEED_EVENT_MINT.as_ref(), // "event_mint" seed
        event.key().as_ref() // "event public key"
      ],
      bump = event.event_mint_bump,
    )]
    pub event_mint: Box<Account<'info, Mint>>, // event mint account PDA

    // payer accepted mint ATA 
    #[account(
        mut,
        // checks the ATA holds the accepted mint
        constraint = payer_accepted_mint_ata.mint == event.accepted_mint,
        constraint = payer_accepted_mint_ata.amount > 0
    )]
    pub payer_accepted_mint_ata: Box<Account<'info, TokenAccount>>, 

    #[account(
        init_if_needed, // create account if doesn't exist
        payer = authority, 
        associated_token::mint = event_mint, // event mint
        associated_token::authority = authority, // token account authority
    )]
    pub payer_event_mint_ata: Box<Account<'info, TokenAccount>>, // payer event mint ATA

    #[account(
        mut,
        seeds = [
            Event::SEED_TREASURY_VAULT.as_ref(), // "treasury_event" seed
            event.key().as_ref() // event public key
        ],
        bump = event.treasury_vault_bump,
      )]
    pub treasury_vault: Box<Account<'info, TokenAccount>>, // event treasury token account

    pub accepted_mint: Box<Account<'info, Mint>>, // accepted mint

    #[account(mut)]
    pub authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    // programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<Sponsor>,
    quantity: u64,
  ) -> Result<()> {
    // quantity * amount
    // calculate decimals
    let multiplier = 10_u64.pow(ctx.accounts.accepted_mint.decimals as u32) as u64;
    // amount * decimals
    let total = quantity.checked_mul(multiplier).unwrap();
    // calculate seeds
    let seeds = [
        ctx.accounts.event.id.as_ref(),
        Event::SEED_EVENT.as_bytes(),
        ctx.accounts.event.authority.as_ref(),
        &[ctx.accounts.event.event_bump],
    ];
    let signer = &[&seeds[..]];
    // Charge the accepted_token amount from payer
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer_accepted_mint_ata.to_account_info(),
                to: ctx.accounts.treasury_vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        total
    )?;
    // Transfer the token
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.event_mint.to_account_info(),
                to: ctx.accounts.payer_event_mint_ata.to_account_info(),
                authority: ctx.accounts.event.to_account_info(),
            },
            signer,
        ),
        quantity,
    )?;
    ctx.accounts.event.sponsors += quantity;
    ctx.accounts.event.treasury_vault_total += total;
    Ok(())
  }