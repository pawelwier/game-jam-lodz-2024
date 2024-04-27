use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::{character::{components::Character, CHAR_SIZE}, game::systems::objects_collide, item::{components::{Item, ITEM_TYPES}, resources::{CharItemInventory, MAX_ITEM_TYPE}, systems::draw_all_area_inventory_items}};

use super::{components::{Area, AreaType}, resources::AreaInventories, AREA_POSITIONS, AREA_SIZE};

pub fn draw_point_areas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) -> () {
    for (x, y, id) in AREA_POSITIONS {
        commands.spawn(
            (
                SpriteBundle {
                    texture: asset_server.load("sprites/area_white.png"),
                    transform: Transform { 
                        translation: Vec3 { x, y, z: 0.0 },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Area {
                    id,
                    area_type: AreaType::Empty
                }
            ),
        );
    }
}

pub fn add_random_area_items(
    commands: Commands,
    asset_server: Res<AssetServer>,
    area_query: Query<(&Transform, &mut Area), With<Area>>,
    mut area_inventories: ResMut<AreaInventories>,
    char_inventory: ResMut<CharItemInventory>
) -> () {
    for i in 0..area_inventories.inventories.len() {
        area_inventories.inventories[i].1.clear_items();
        for item_type in ITEM_TYPES {
            let num = thread_rng().gen_range(0..MAX_ITEM_TYPE);
            for _ in 0..=num {
                area_inventories.inventories[i].1.add_item(Item { item_type });
            }
        }
    }

    draw_all_area_inventory_items(
        commands,
        asset_server,
        area_query,
        &area_inventories
    );

    update_area_types(
        area_inventories,
        &char_inventory,
    );
}

fn update_area_types(
    mut area_inventories: ResMut<AreaInventories>,
    char_inventory: &CharItemInventory
) -> () {
    for i in 0..area_inventories.inventories.len() {
        let mut can_get_points: bool = true; 
        let (_, items, _) = &area_inventories.inventories[i];
        for item_type in ITEM_TYPES {
            let char_type_count = char_inventory.inventory.get_item_type_count(item_type);
            let area_type_count = items.get_item_type_count(item_type);
            if area_type_count > char_type_count { can_get_points = false; }
        }
        if can_get_points {
            area_inventories.inventories[i].2 = AreaType::Available;
        } else {
            area_inventories.inventories[i].2 = AreaType::Forbidden;
        }
    }
}

pub fn check_step_on_area(
    area_query: Query<(&Transform, &Area), With<Area>>,
    char_query: Query<&Transform, With<Character>>,
    area_inventories: ResMut<AreaInventories>
) -> () {
    let char_transform = char_query.get_single().unwrap();

    for (area_transform, area) in area_query.iter() {
        if objects_collide(
            (
                (char_transform.translation.x, char_transform.translation.y),
                (CHAR_SIZE, CHAR_SIZE)
            ),
            (
                (area_transform.translation.x, area_transform.translation.y),
                (AREA_SIZE, AREA_SIZE)
            ),
        ) {
            let area_item = area_inventories.inventories
                .iter()
                .find(|a| { a.0 == area.id })
                .unwrap();

            if area_item.2 == AreaType::Available {
                println!("{:?}", area.id);
            }
        }
    }
}