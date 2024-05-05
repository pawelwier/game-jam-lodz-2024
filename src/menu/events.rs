use bevy::prelude::*;

use crate::game::GameState;

#[derive(Event)]
pub struct FinalMenuClosed {
    pub new_state: GameState
}