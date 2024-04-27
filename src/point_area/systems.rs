use bevy::prelude::*;

use crate::item::resources::ItemInventory;

use super::{components::{Area, AreaType}, AREA_POSITIONS};

pub fn draw_point_areas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) -> () {
    for pos in AREA_POSITIONS {
        commands.spawn(
            (
                SpriteBundle {
                    texture: asset_server.load("sprites/area_white.png"),
                    transform: Transform { 
                        translation: Vec3 { x: pos.0, y: pos.1, z: 0.0 },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Area {
                    area_type: AreaType::Empty,
                    area_inventory: ItemInventory {
                        items: Vec::new()
                    }
                }
            ),
        );
    }
}