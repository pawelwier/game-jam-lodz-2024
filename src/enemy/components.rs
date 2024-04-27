use bevy::prelude::*;

#[derive(Component)]
pub struct Ufo {
    pub ufo_type: UfoType
}

#[derive(Component)]
pub struct Laser {
    pub direction_x: f32,
    pub direction_y: f32
}

pub enum UfoType {
    TopDown,
    LeftRight
}