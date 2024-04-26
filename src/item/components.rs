use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ItemType {
    A,
    B,
    C
}

#[derive(Component, Copy, Clone)]
pub struct Item {
    pub item_type: ItemType,
}

impl Item {
    pub fn get_icon(&self) -> String {
        let icon: &str =  if self.item_type == ItemType::A {
            "a"
        } else if self.item_type == ItemType::B {
            "b"
        } else { "c" };

        return icon.to_string();
    }
}