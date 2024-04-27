use bevy::prelude::*;

use crate::item::{components::Item, resources::ItemInventory};

use super::{components::{AreaId, AreaType}, RELOAD_AREAS_TIMER};

#[derive(Resource)]
pub struct AreaInventories {
    pub inventories: [(AreaId, ItemInventory, AreaType); 4]
}

impl Default for AreaInventories {
    fn default() -> Self {
        AreaInventories {
            inventories: [
                (AreaId::BottomLeft, ItemInventory { items: Vec::<Item>::new()}, AreaType::Empty ),
                (AreaId::BottomRight, ItemInventory { items: Vec::<Item>::new()}, AreaType::Empty ),
                (AreaId::TopLeft, ItemInventory { items: Vec::<Item>::new()}, AreaType::Empty ),
                (AreaId::TopRight, ItemInventory { items: Vec::<Item>::new()}, AreaType::Empty ),
            ]
        }
    }
}

#[derive(Resource)]
pub struct ReloadAreasTimer {
    pub timer: Timer
}

impl Default for ReloadAreasTimer {
    fn default() -> ReloadAreasTimer {
        ReloadAreasTimer { timer: Timer::from_seconds(RELOAD_AREAS_TIMER, TimerMode::Repeating) }
    }
}