use bevy::prelude::*;

use super::components::{Item, ItemType};

pub const INVENTORY_BG_WIDTH: f32 = 200.0;
pub const INVENTORY_BG_HEIGHT: f32 = 300.0;
const MAX_ITEM_TYPE: usize = 3;

#[derive(Resource)]
pub struct CharItemInventory {
    pub inventory: ItemInventory
}

pub struct ItemInventory {
    pub items: Vec<Item>
}

impl Default for CharItemInventory {
    fn default() -> Self {
        CharItemInventory {
            inventory: ItemInventory {
                items: Vec::<Item>::new()
            }
        }
    }
}

impl ItemInventory {
    pub fn add_item(&mut self, item: Item) -> () {
        self.items.push(item);
    }

    pub fn get_items_of_type(&self, item_type: ItemType) -> Vec<Item> {
        self.items
        .iter()
        .map(|item| *item)
        .filter(|item| { item.item_type == item_type })
        .collect()
    }


    pub fn get_item_type_count(&self, item_type: ItemType) -> usize {
        self.items
            .iter()
            .filter(|item| { item.item_type == item_type })
            .count()
    }

    pub fn can_add_item_type(&self, item_type: ItemType) -> bool {
        self.get_item_type_count(item_type) < MAX_ITEM_TYPE
    }
}