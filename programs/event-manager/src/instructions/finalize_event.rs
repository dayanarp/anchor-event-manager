use {crate::collections::event::Event, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct FinalizeEvent<'info> {
    #[account(mut)]
    pub event: Box<Account<'info, Event>>, // event account

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<FinalizeEvent>
) -> Result<()> {
    ctx.accounts.event.active = false;
    Ok(())
}