use bevy::prelude::*;

#[derive(Resource)]
pub struct CharMovement {
    pub state: MovementState,
    direction: Vec3,
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
            speed: 400.0,
            direction: Vec3::ZERO
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

    pub fn set_direction(&mut self, direction: Vec3) -> () {
        self.direction = direction;
    }

    pub fn move_char(&mut self) -> () {
        self.set_state(MovementState::Move);
    }

    pub fn stop_char(&mut self) -> () {
        self.set_state(MovementState::Idle);
    }

    pub fn is_moving(&mut self) -> bool {
        self.state == MovementState::Move
    }
}