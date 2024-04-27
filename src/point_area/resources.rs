use bevy::prelude::*;

use crate::item::{components::Item, resources::ItemInventory};

use super::components::AreaId;

#[derive(Resource)]
pub struct AreaInventories {
    pub inventories: [(AreaId, ItemInventory); 4]
}

impl Default for AreaInventories {
    fn default() -> Self {
        AreaInventories {
            inventories: [
                (AreaId::BottomLeft, ItemInventory { items: Vec::<Item>::new() }),
                (AreaId::BottomRight, ItemInventory { items: Vec::<Item>::new() }),
                (AreaId::TopLeft, ItemInventory { items: Vec::<Item>::new() }),
                (AreaId::TopRight, ItemInventory { items: Vec::<Item>::new() })
            ]
        }
    }
}