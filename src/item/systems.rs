use std::ops::Index;

use bevy::prelude::*;

use super::{components::{Item, ItemType, ITEM_HEIGHT, ITEM_WIDTH}, resources::ItemInventory};

pub fn spawn_inventory_item(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: (f32, f32),
    icon: String
) -> () {
    let icon_path = "sprites/".to_string() + &icon;
    commands.spawn(
        (
            SpriteBundle {
                texture: asset_server.load(icon_path),
                transform: Transform { 
                    translation: Vec3 { x: position.0, y: position.1, z: 0.0 },
                    ..Default::default()
                },
                ..Default::default()
            },
        )
    );
}

pub fn draw_inventory_bg(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) -> () {
    commands.spawn(
        (
            SpriteBundle {
                texture: asset_server.load("sprites/inventory_bg.png"),
                transform: Transform { 
                    translation: Vec3 { x: 1050.0, y: 450.0, z: 0.0 },
                    ..Default::default()
                },
                ..Default::default()
            },
        )
    );
}

pub fn draw_inventory_items(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    inventory: Res<ItemInventory>
) -> () {
    let types: [ItemType; 3] = [ItemType::C, ItemType::B, ItemType::A];

    let row_initial_position: (f32, f32) = (1000.0, 450.0);
    const MARGIN: f32 = 15.0;

    types
        .iter()
        .for_each(|item_type| {
            let type_index = types.iter().position(|i_type| item_type == i_type).unwrap();
            let item_count = inventory.get_item_type_count(*item_type);

            if item_count > 0 {
                let item: Item = inventory.get_items_of_type(*item_type)[0];
                for i in 0..item_count {
                    spawn_inventory_item(
                        &mut commands,
                        &asset_server,
                        (row_initial_position.0 + i as f32 * (ITEM_WIDTH + MARGIN), row_initial_position.1 + (ITEM_HEIGHT + MARGIN) * type_index as f32),
                        item.get_icon()
                    )
                }
            }
        });
}


// pub fn draw_inventory_items(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     inventory: Res<ItemInventory>
// ) -> () {
//     // TODO: refactor, super ugly
//     let item_a_count = inventory.get_item_type_count(ItemType::A);
//     let item_b_count  = inventory.get_item_type_count(ItemType::B);
//     let item_c_count = inventory.get_item_type_count(ItemType::C);

//     for i in 0..item_a_count {
//         spawn_inventory_item(
//             &mut commands,
//             &asset_server,
//             (1000.0 + i as f32 * 50.0, 450.0),
//             "item_a.png".to_string()
//         )
//     }

//     for i in 0..item_b_count {
//         spawn_inventory_item(
//             &mut commands,
//             &asset_server,
//             (1000.0 + i as f32 * 50.0, 500.0),
//             "item_b.png".to_string()
//         )
//     }

//     for i in 0..item_c_count {
//         spawn_inventory_item(
//             &mut commands,
//             &asset_server,
//             (1000.0 + i as f32 * 50.0, 550.0),
//             "item_c.png".to_string()
//         )
//     }
// }