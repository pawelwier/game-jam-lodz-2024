use crate::game::{MOVE_AREA_POS, MOVE_AREA_SIZE};

use self::components::AreaId;

pub mod components;
pub mod systems;
pub mod resources;
pub mod events;

pub const AREA_SIZE: f32 = 80.;
pub const AREA_POSITIONS: [(f32, f32, AreaId); 4] = [
    (MOVE_AREA_POS.0 + AREA_SIZE / 2., MOVE_AREA_POS.1 + AREA_SIZE / 2., AreaId::BottomLeft),
    (MOVE_AREA_POS.0 + MOVE_AREA_SIZE.0 - AREA_SIZE / 2., MOVE_AREA_POS.1 + AREA_SIZE / 2., AreaId::BottomRight),
    (MOVE_AREA_POS.0 + AREA_SIZE / 2., MOVE_AREA_POS.1 + MOVE_AREA_SIZE.1 - AREA_SIZE / 2., AreaId::TopLeft),
    (MOVE_AREA_POS.0 + MOVE_AREA_SIZE.0 - AREA_SIZE / 2., MOVE_AREA_POS.1 + MOVE_AREA_SIZE.1 - AREA_SIZE / 2., AreaId::TopRight)
];

pub const RELOAD_AREAS_TIMER: f32 = 6.0; 