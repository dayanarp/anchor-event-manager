use {crate::collections::Event, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct CloseEvent<'info> {
    #[account(
        mut,
        seeds = [
            Event::SEED_EVENT.as_ref(), // "event" seed
            // checks only authority provided can close the event
            authority.key().as_ref() // authority public key
        ],
        bump = event.event_bump,
    )]
    pub event: Box<Account<'info, Event>>, // event account

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<CloseEvent>
) -> Result<()> {
    ctx.accounts.event.active = false;
    Ok(())
}