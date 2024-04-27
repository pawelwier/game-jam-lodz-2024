use bevy::prelude::*;

use super::{LEFT_RIGHT_TIMER, UP_DOWN_TIMER};

#[derive(Resource)]
pub struct LaserTimer {
    pub up_down_timer: Timer,
    pub left_right_timer: Timer
}

impl Default for LaserTimer {
    fn default() -> LaserTimer {
        LaserTimer { 
            up_down_timer: Timer::from_seconds(UP_DOWN_TIMER, TimerMode::Repeating),
            left_right_timer: Timer::from_seconds(LEFT_RIGHT_TIMER, TimerMode::Repeating)
        }
    }
}