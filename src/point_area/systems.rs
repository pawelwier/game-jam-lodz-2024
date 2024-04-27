use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::item::{components::{Item, ITEM_TYPES}, resources::MAX_ITEM_TYPE};

use super::{components::{Area, AreaType}, resources::AreaInventories, AREA_POSITIONS};

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
    mut area_inventories: ResMut<AreaInventories>
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
}