use {
    crate::{collections::event::Event, utils::calculate_total}, 
    anchor_lang::prelude::*, 
    anchor_spl::{
        associated_token::*, token::*
    },
};

#[derive(Accounts)]
pub struct SponsorEvent<'info> {
    #[account(mut)]
    pub event: Box<Account<'info, Event>>, // event account

    #[account(
      mut,
      seeds = [
        Event::SEED_EVENT_MINT.as_bytes(), // "event_mint" seed
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
            Event::SEED_TREASURY_VAULT.as_bytes(), // "treasury_event" seed
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
    ctx: Context<SponsorEvent>,
    quantity: u64,
  ) -> Result<()> {

    // total to transfer = quantity * price * accepted mint decimals
    let total = calculate_total(quantity, ctx.accounts.event.token_price, ctx.accounts.accepted_mint.decimals);

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
        total,
    )?;
    // Mint the token
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
    ctx.accounts.event.tokens_sold += quantity;
    ctx.accounts.event.total_sponsors += 1;
    ctx.accounts.event.current_sponsors +=1;
    Ok(())
  }