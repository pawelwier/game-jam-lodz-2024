pub mod components;
pub mod resources;
pub mod systems;

pub const CHAR_FRAME_COUNT: usize = 6;
pub const CHAR_SIZE: f32 = 64.;

pub const CHAR_SPEED: f32 = 450.;
pub const CHAR_SPEED_PENALTY: f32 = 40.;

pub const CHAR_INIT_POS: (f32, f32) = (300., 400.);