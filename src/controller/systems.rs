use bevy::prelude::*;

use crate::item::{components::{Item, ItemType}, resources::ItemInventory};

use super::utils::is_key_just_pressed;

pub fn handle_add_item(
    input: Res<ButtonInput<KeyCode>>,
    mut item_inventory: ResMut<ItemInventory>
) -> () {
    let input_type_map: [(KeyCode, ItemType); 3] = [
        (KeyCode::Digit1, ItemType::A),
        (KeyCode::Digit2, ItemType::B),
        (KeyCode::Digit3, ItemType::C)
    ];
    
    for (code, item_type) in input_type_map {
        if is_key_just_pressed(&input, code)
            && item_inventory.can_add_item_type(item_type)    
        {
            item_inventory.add_item(Item { item_type });
        }
    }   

    println!("{}", item_inventory.items.len());
}