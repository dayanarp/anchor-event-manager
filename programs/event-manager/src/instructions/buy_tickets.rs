use {
    crate::collections::event::Event, 
    anchor_lang::prelude::*, 
    anchor_spl::token::*,
    crate::utils::calculate_total
};

#[derive(Accounts)]
pub struct BuyTickets<'info> {
    #[account(
        mut,
        constraint = event.active == true @ ErrorCode::EventClosed
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
            Event::SEED_GAIN_VAULT.as_bytes(), // "gain_vault" seed
            event.key().as_ref() // event public key
        ],
        bump = event.gain_vault_bump,
      )]
    pub gain_vault: Box<Account<'info, TokenAccount>>, // event gain vault account

    pub accepted_mint: Box<Account<'info, Mint>>, // accepted mint

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

    // total to transfer = quantity * price * accepted mint decimals
    let total = calculate_total(quantity, ctx.accounts.event.ticket_price, ctx.accounts.accepted_mint.decimals);
    
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
        total, // amount to charge
    )?;
    ctx.accounts.event.tickets_sold += quantity;
    Ok(())
  }

#[error_code]
pub enum ErrorCode {
    #[msg("You can't buy tickets, the Event is already closed")]
    EventClosed,
}