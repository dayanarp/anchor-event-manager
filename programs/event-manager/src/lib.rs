use { anchor_lang::prelude::*, crate::instructions::*} ;

mod collections;
mod instructions;

declare_id!("3FAqGnGM4DfcAGvp3V8FbUmgjikt1jN49ppivh2zWUuE");

#[program]
pub mod event_manager {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        name: String,
        ticket_price: u64,
    ) -> Result<()> {
        instructions::create_event::handle(ctx, name, ticket_price)
    }
}

