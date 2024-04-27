use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub points: usize
}

impl Default for Score {
    fn default() -> Self {
        Score { points: 0 }
    }
}