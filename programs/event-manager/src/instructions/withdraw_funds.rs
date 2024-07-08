use {
    crate::collections::Event, 
    anchor_lang::prelude::*, 
    anchor_spl::{
        associated_token::*, token::*
    }
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct WithdrawFunds<'info> {
    #[account(
        mut,
        seeds = [
            Event::SEED_EVENT.as_ref(), // "event" seed
            // checks only authority provided can withdraw
            authority.key().as_ref() // authority public key
        ],
        bump = event.event_bump,
    )]
    pub event: Box<Account<'info, Event>>, // event account

    pub accepted_mint: Box<Account<'info, Mint>>, // accepted mint

    #[account(
        init_if_needed, // create account if doesn't exist
        payer = authority, 
        associated_token::mint = accepted_mint, // event mint
        associated_token::authority = authority, // token account authority
    )]
    // ATA of the authorithy withdrawing funds
    pub authotiry_accepted_mint_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            Event::SEED_TREASURY_VAULT.as_ref(), // "treasury_vault" seed
            event.key().as_ref() // event public key
        ],
        bump = event.treasury_vault_bump,
        constraint = treasury_vault.amount >= amount
      )]
    pub treasury_vault: Box<Account<'info, TokenAccount>>, // event treasury account

    #[account(mut)]
    pub authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<WithdrawFunds>,
    amount: u64,
) -> Result<()> {
    let seeds = [
        // seeds from event PDA 
        Event::SEED_EVENT.as_bytes(), // "event" seed
        ctx.accounts.event.authority.as_ref(), // authority public key
        &[ctx.accounts.event.event_bump], // bump
    ];
    let signer = &[&seeds[..]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury_vault.to_account_info(), // event treasury vault
                to: ctx.accounts.authotiry_accepted_mint_ata.to_account_info(), // authority ATA
                authority: ctx.accounts.event.to_account_info(), // authority of the vault (the event PDA)
            },
            signer, // event PDA seeds
        ),
        amount, // amount to withdraw
    )?;
    Ok(())
}