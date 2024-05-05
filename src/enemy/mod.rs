use crate::game::{MOVE_AREA_MARGIN, MOVE_AREA_POS, MOVE_AREA_SIZE};

use self::components::UfoType;

pub mod components;
pub mod resources;
pub mod systems;

pub const LASER_SPEED: f32 = 330.;
pub const LASER_SIZE: f32 = 14.0;

const UP_DOWN_TIMER: f32 = 2.6;
const LEFT_RIGHT_TIMER: f32 = 3.2;

pub const UFO_BOTTOM_POS: (f32, f32) = (MOVE_AREA_POS.0 + MOVE_AREA_SIZE.0 / 2., MOVE_AREA_MARGIN);
pub const UFO_TOP_POS: (f32, f32) = (MOVE_AREA_POS.0 + MOVE_AREA_SIZE.0 / 2., MOVE_AREA_SIZE.1 + MOVE_AREA_MARGIN);
pub const UFO_LEFT_POS: (f32, f32) = (MOVE_AREA_POS.0, MOVE_AREA_MARGIN + MOVE_AREA_SIZE.1 / 2.);
pub const UFO_RIGHT_POS: (f32, f32) = (MOVE_AREA_POS.0 + MOVE_AREA_SIZE.0, MOVE_AREA_MARGIN + MOVE_AREA_SIZE.1 / 2.);

pub const UFO_POSITIONS: [(f32, f32, UfoType); 4] = [
    (UFO_BOTTOM_POS.0,UFO_BOTTOM_POS.1, UfoType::TopDown),
    (UFO_TOP_POS.0,UFO_TOP_POS.1, UfoType::TopDown),
    (UFO_LEFT_POS.0,UFO_LEFT_POS.1, UfoType::LeftRight),
    (UFO_RIGHT_POS.0,UFO_RIGHT_POS.1, UfoType::LeftRight),
];