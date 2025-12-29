pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2LX6yv32GieA7v5rSPSFrxFknvv5BVEgGG7AU3W27Xoj");

#[program]
pub mod ws_swap {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        instructions::make_offer::send_offer_token_vaults(&ctx, token_a_offered_amount)?;
        instructions::make_offer::save_offer(ctx, id, token_b_wanted_amount)
        //Ok(())
    }
    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        //instructions::take_offer::send_offer_token_vaults(&ctx,token_b_offered_amount)?;
        //instructions::take_offer::save_offer(ctx,id,token_b_offered_amount)
        instructions::take_offer::send_wanted_token_to_maker(&ctx)?;
        instructions::take_offer::withdraw_and_close_valut(ctx)?;
        Ok(())
    }
}
