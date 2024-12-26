use { anchor_lang::prelude::*, crate::instructions::*} ;

mod collections;
mod instructions;
mod utils;

declare_id!("8iSfAPbssiTxpeTYVPx4syA6VUgo2tur9nvmks4oRMsq");

#[program]
pub mod event_manager {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        id: String,
        name: String,
        description: String,
        ticket_price: f64,
        token_price: f64
    ) -> Result<()> {
        instructions::create_event::handle(ctx, id, name, description, ticket_price, token_price)
    }

    pub fn delete_event(ctx: Context<DeleteEvent>) -> Result<()>{
        instructions::delete_event::handle(ctx)
    }

     // sponsor event (get event mint tokens)
     pub fn sponsor_event (
        ctx: Context<SponsorEvent>,
        quantity: u64,
    ) -> Result<()> {
        instructions::sponsor_event::handle(ctx, quantity)
    }

    // buy tickets
    pub fn buy_tickets (
        ctx: Context<BuyTickets>,
        quantity: u64,
    ) -> Result<()> {
        instructions::buy_tickets::handle(ctx, quantity)
    }

     // withdraw funds
     pub fn withdraw_funds(
        ctx: Context<WithdrawFunds>,
        amount: f64,
    ) -> Result<()> {
        instructions::withdraw_funds::handle(ctx, amount)
    }

    // close event
    pub fn close_event (
        ctx: Context<FinalizeEvent>
    ) -> Result<()> {
        instructions::finalize_event::handle(ctx)
    }

    // withdraw earnings
    pub fn withdraw_earnings(
        ctx: Context<WithdrawEarnings>
    ) -> Result<()> {
        instructions::withdraw_earnings::handle(ctx)
    }
}

