use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameState {
    pub current_game_id: u64,
    pub cards: [u8; 5],
    pub nfts_rewards: [u8; 5],
    pub life: u8,
    pub bump: u8,
    pub is_active: bool,
}
