use bevy::prelude::*;

use super::CHAR_SPEED;

#[derive(Resource)]
pub struct CharMovement {
    pub state: MovementState,
    speed: f32
}

#[derive(Debug, PartialEq)]
pub enum MovementState {
    Idle,
    Move
}

impl Default for CharMovement {
    fn default() -> CharMovement {
        CharMovement {
            state: MovementState::Idle,
            speed: CHAR_SPEED
        }
    }
}

impl CharMovement {
    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: f32) -> () {
        self.speed = speed;
    }

    pub fn set_state(&mut self, state: MovementState) -> () {
        self.state = state;
    }
}