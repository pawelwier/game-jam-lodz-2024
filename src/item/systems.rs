use bevy::prelude::*;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::{components::{Item, ItemType, ITEM_HEIGHT, ITEM_WIDTH}, resources::{CharItemInventory, INVENTORY_BG_WIDTH}};

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
                    translation: Vec3 { 
                        x: WINDOW_WIDTH - INVENTORY_BG_WIDTH / 2.0 - 50.0,
                        y: WINDOW_HEIGHT / 2.0, 
                        z: 0.0
                    },
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
    char_inventory: Res<CharItemInventory>
) -> () {
    let types: [ItemType; 3] = [
        ItemType::C, ItemType::B, ItemType::A
    ];

    let row_initial_position: (f32, f32) = (
        WINDOW_WIDTH - INVENTORY_BG_WIDTH,
        WINDOW_HEIGHT / 2.0
    );
    const MARGIN: f32 = 15.0;

    types
        .iter()
        .for_each(|item_type| {
            let type_index = types.iter().position(|i_type| item_type == i_type).unwrap();
            let item_count = char_inventory.inventory.get_item_type_count(*item_type);

            if item_count > 0 {
                let item: Item = char_inventory.inventory.get_items_of_type(*item_type)[0];
                for i in 0..item_count {
                    spawn_inventory_item(
                        &mut commands,
                        &asset_server,
                        (
                            row_initial_position.0 + i as f32 * (ITEM_WIDTH + MARGIN), 
                            row_initial_position.1 + (ITEM_HEIGHT + MARGIN) * type_index as f32
                        ),
                        item.get_icon()
                    )
                }
            }
        });
}
