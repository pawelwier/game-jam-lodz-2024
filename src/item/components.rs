use bevy::prelude::*;

pub const ITEM_WIDTH : f32 = 34.0;
pub const ITEM_HEIGHT : f32 = 33.0;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ItemType {
    A,
    B,
    C
}

pub const ITEM_TYPES: [ItemType; 3] = [
    ItemType::C, ItemType::B, ItemType::A
];

#[derive(Component, Copy, Clone)]
pub struct Item {
    pub item_type: ItemType,
}

#[derive(Component)]
pub struct CharacterItem {}

impl Item {
    pub fn get_icon(&self) -> String {
        let icon: &str =  if self.item_type == ItemType::A {
            "item_a.png"
        } else if self.item_type == ItemType::B {
            "item_b.png"
        } else { 
            "item_c.png"
        };

        return icon.to_string();
    }
}