use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod systems;
pub mod helpers;


pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);