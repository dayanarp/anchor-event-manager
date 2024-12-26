use {crate::collections::event::Event, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct DeleteEvent<'info> {
    #[account(mut, close = authority)]
    pub event: Box<Account<'info, Event>>, // event account

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handle(
    ctx: Context<DeleteEvent>
) -> Result<()> {
    Ok(())
}