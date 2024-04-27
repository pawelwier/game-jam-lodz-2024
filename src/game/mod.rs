use crate::item::resources::INVENTORY_BG_WIDTH;

pub mod components;
pub mod systems;

pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 900.0;

pub const MOVE_AREA_MARGIN: f32 = 50.0;
pub const MOVE_AREA_POS: (f32, f32) = (MOVE_AREA_MARGIN, MOVE_AREA_MARGIN);
pub const MOVE_AREA_SIZE: (f32, f32) = (
    WINDOW_WIDTH - INVENTORY_BG_WIDTH - 3.0 * MOVE_AREA_MARGIN,
    WINDOW_HEIGHT - 2.0 * MOVE_AREA_MARGIN
);