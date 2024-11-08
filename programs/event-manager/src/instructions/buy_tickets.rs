use {
    crate::collections::Event, 
    anchor_lang::prelude::*, 
    anchor_spl::token::*
};

#[derive(Accounts)]
pub struct BuyTickets<'info> {
    #[account(
        mut,
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
    // calculate amount to charge (quantity * token_price)
    let amount = ctx
        .accounts
        .event
        .ticket_price
        .checked_mul(quantity)
        .unwrap();
    // calculate decimals
    let multiplier = 10_u64.pow(ctx.accounts.accepted_mint.decimals as u32) as u64;
    // amount * decimals
    let total = amount.checked_mul(multiplier).unwrap();
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
    ctx.accounts.event.gain_vault_total += total;
    ctx.accounts.event.tickets_sold += quantity;
    Ok(())
  }

#[error_code]
pub enum ErrorCode {
    #[msg("You can't buy tickets, the Event is already closed")]
    EventClosed,
}

