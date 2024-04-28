use bevy::prelude::*;

use crate::item::resources::INVENTORY_BG_WIDTH;

pub mod components;
pub mod systems;
pub mod resources;

pub const WINDOW_WIDTH: f32 = 1200.;
pub const WINDOW_HEIGHT: f32 = 900.;

pub const MOVE_AREA_MARGIN: f32 = 50.;
pub const MOVE_AREA_POS: (f32, f32) = (MOVE_AREA_MARGIN, MOVE_AREA_MARGIN);
pub const MOVE_AREA_SIZE: (f32, f32) = (
    WINDOW_WIDTH - INVENTORY_BG_WIDTH - 3.0 * MOVE_AREA_MARGIN,
    WINDOW_HEIGHT - 2. * MOVE_AREA_MARGIN
);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    LoadInventory,
    GetReady,
    Play,
    GameOver,
    Completed,
}