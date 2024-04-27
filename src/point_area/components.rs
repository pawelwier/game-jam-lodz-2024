use bevy::prelude::*;

use crate::item::resources::ItemInventory;

pub enum AreaType {
    Empty,
    Available,
    Forbidden,
    Used
}

#[derive(PartialEq)]
pub enum AreaId {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight
}

#[derive(Component)]
pub struct Area {
    pub id: AreaId,
    pub area_type: AreaType,
    // pub area_inventory: ItemInventory
}

impl Area {

}