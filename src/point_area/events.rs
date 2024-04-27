use bevy::prelude::*;

use super::components::AreaId;

#[derive(Event)]
pub struct AreaCaptured {
    pub id: AreaId
}