use anchor_lang::prelude::*;

use crate::GameState;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        init,
        payer=player,
        seeds = [b"game_state", seed.to_le_bytes().as_ref(), player.key().as_ref()],
        space = 8 + GameState::INIT_SPACE,
        bump

    )]
    pub game_state: Account<'info, GameState>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub validator: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn init_game(&mut self, seed: u64, bumps: &InitializeBumps) -> Result<()> {
        // get 5 random numbers from vrf
        // initiate the vrf request here in this ixn and handle callback in vrf_callback
        // Ask vrf for a number between 0 and 9999 (inclusive).
        // That’s 10,000 possible seeds → 13 bits of entropy.
        // It’s simple to pick and share, but still random enough that your derived 5 numbers will look unpredictable.
        // delegate game state pda to er
        let cards = [1, 2, 3, 4, 5];
        self.game_state.set_inner(GameState {
            bump: bumps.game_state,
            current_game_id: seed,
            cards,               // todo: it should empty here, it's updated in the vrf_callback
            nfts_rewards: cards, // todo: incorrect rn, remove later
            life: 3,
            is_active: true,
        });

        Ok(())
    }
}
