use {
    crate::collections::Event, 
    anchor_lang::prelude::*, 
    anchor_spl::token::*
};

#[derive(Accounts)]
pub struct BuyTickets<'info> {
    #[account(
        mut,
        // check if the event is still active
        seeds = [
            Event::SEED_EVENT.as_ref(), // "event" seed
            // checks only authority provided can withdraw
            event.authority.key().as_ref() // authority public key
        ],
        bump = event.event_bump,
        constraint = event.active == true @ ErrorCode::EventClosed, 
    )] 
    pub event: Box<Account<'info, Event>>, // event account

    // payer accepted mint ATA
    #[account(
        mut,
        // checks the ATA holds the accepted mint
        constraint = payer_accepted_mint_ata.mint == event.accepted_mint,
        constraint = payer_accepted_mint_ata.amount > 0
    )]
    pub payer_accepted_mint_ata: Box<Account<'info, TokenAccount>>, 

    #[account(
        mut,
        seeds = [
            Event::SEED_GAIN_VAULT.as_ref(), // "gain_vault" seed
            event.key().as_ref() // event public key
        ],
        bump = event.gain_vault_bump,
      )]
    pub gain_vault: Box<Account<'info, TokenAccount>>, // event gain vault account

    #[account(mut)]
    pub authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<BuyTickets>,
    quantity: u64,
  ) -> Result<()> {
    // calculate amount to charge (quantity * token_price)
    let amount = ctx
        .accounts
        .event
        .ticket_price
        .checked_mul(quantity)
        .unwrap();
    // Charge the amount
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer_accepted_mint_ata.to_account_info(), // payer accepted mint ata
                to: ctx.accounts.gain_vault.to_account_info(), // event gain vault
                authority: ctx.accounts.authority.to_account_info(), // payer (authority of the from account)
            },
        ),
        amount, // amount to charge
    )?;
    Ok(())
  }

#[error_code]
pub enum ErrorCode {
    #[msg("You can't buy tickets, the Event is already closed")]
    EventClosed,
}

