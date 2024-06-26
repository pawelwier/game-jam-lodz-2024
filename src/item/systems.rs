use bevy::prelude::*;

use crate::{
    point_area::{
        components::{
            Area, AreaId, AreaType
        }, 
        resources::AreaInventories
    }, 
    WINDOW_HEIGHT, 
    WINDOW_WIDTH
};

use super::{
    components::{
        CharacterItem, 
        Item, 
        ItemType, 
        ITEM_HEIGHT, 
        ITEM_TYPES, 
        ITEM_WIDTH
    }, 
    resources::{
        CharItemInventory, 
        ItemInventory, 
        INVENTORY_BG_WIDTH
    }
};

pub fn spawn_inventory_item(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: (f32, f32),
    icon: String,
    item_type: ItemType,
    scale: f32,
    is_char_inventory: bool
) -> () {
    let icon_path: String = "sprites/".to_string() + &icon;
    let bundle: SpriteBundle = SpriteBundle {
        texture: asset_server.load(icon_path),
        transform: Transform { 
            translation: Vec3 { x: position.0, y: position.1, z: 0. },
            scale: Vec3 { x: scale, y: scale, z: 0. },
            ..Default::default()
        },
        ..Default::default()
    };

    if is_char_inventory {
        commands.spawn(
            (
                bundle,
                Item {
                    item_type
                },
                CharacterItem {}
            )
        );
    } else {
        commands.spawn(
            (
                bundle,
                Item {
                    item_type
                }
            )
        );
    }

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
                        x: WINDOW_WIDTH - INVENTORY_BG_WIDTH / 2. - 50.,
                        y: WINDOW_HEIGHT / 2., 
                        z: 0.
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
        )
    );
}

pub fn draw_all_area_inventory_items(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    area_query: Query<(&Transform, Entity, &mut Area), With<Area>>,
    area_inventories: &ResMut<AreaInventories>
) -> () {
    for (area_transform, _, area) in area_query.iter() {
        draw_area_inventory_items(
            &mut commands,
            &asset_server,
            area,
            area_transform,
            &area_inventories.inventories
        )
    }
}

pub fn draw_area_inventory_items(
    mut commands: &mut Commands,
    asset_server: &AssetServer,
    area: &Area,
    area_transform: &Transform,
    area_inventories: &[(AreaId, ItemInventory, AreaType); 4]
) -> () {
    let inventory = area_inventories
        .iter()
        .find(|item| { item.0 == area.id })
        .unwrap();

    draw_inventory_items(
        &mut commands,
        asset_server,
        &inventory.1,
        (
            area_transform.translation.x - ITEM_WIDTH / 2.,
            area_transform.translation.y - ITEM_HEIGHT / 2.
        ),
        0.4,
        false
    )

}

pub fn draw_char_inventory_items(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    char_inventory: Res<CharItemInventory>
) -> () {
    let row_initial_position: (f32, f32) = (
        WINDOW_WIDTH - INVENTORY_BG_WIDTH,
        WINDOW_HEIGHT / 2.
    );

    draw_inventory_items(
        &mut commands,
        &asset_server,
        &char_inventory.inventory,
        row_initial_position,
        1.,
        true
    )
}

pub fn draw_inventory_items(
    commands: &mut Commands,
    asset_server: &AssetServer,
    inventory: &ItemInventory,
    row_initial_position: (f32, f32),
    scale: f32,
    is_char_inventory: bool
) -> () {
    ITEM_TYPES
        .iter()
        .for_each(|item_type| {
            let type_index = ITEM_TYPES.iter().position(|i_type| item_type == i_type).unwrap();
            let item_count = inventory.get_item_type_count(*item_type);

            if item_count > 0 {
                let item: Item = inventory.get_items_of_type(*item_type)[0];
                for i in 0..item_count {
                    spawn_inventory_item(
                        commands,
                        asset_server,
                        (
                            row_initial_position.0 + i as f32 * ITEM_WIDTH * 1.4 * scale, 
                            row_initial_position.1 + ITEM_HEIGHT * 1.4 * type_index as f32 * scale
                        ),
                        item.get_icon(),
                        *item_type,
                        scale,
                        is_char_inventory
                    )
                }
            }
        });
}
