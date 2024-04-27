use bevy::prelude::*;

use crate::item::resources::ItemInventory;

pub enum AreaType {
    Empty,
    Available,
    Forbidden,
    Used
}

#[derive(Component)]
pub struct Area {
    pub area_type: AreaType,
    pub area_inventory: ItemInventory
}

impl Area {

}