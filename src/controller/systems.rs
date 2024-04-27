use bevy::prelude::*;

use crate::{
    animation::systems::set_atlas_layout, character::{
        components::Character, resources::{
            CharMovement, MovementState
        }, CHAR_SIZE
    }, game::{GameState, MOVE_AREA_POS, MOVE_AREA_SIZE}, item::{
        components::{
            Item, 
            ItemType
        }, 
        resources::{CharItemInventory, ItemInventory}
    }
};

use super::utils::{is_key_just_pressed, is_key_pressed};

pub fn handle_add_item(
    input: Res<ButtonInput<KeyCode>>,
    mut item_inventory: ResMut<CharItemInventory>,
    mut app_state_next_state: ResMut<NextState<GameState>>
) -> () {
    let input_type_map: [(KeyCode, ItemType); 3] = [
        (KeyCode::Digit1, ItemType::A),
        (KeyCode::Digit2, ItemType::B),
        (KeyCode::Digit3, ItemType::C)
    ];

    for (code, item_type) in input_type_map {
        if is_key_just_pressed(&input, code)
            && item_inventory.inventory.can_add_item_type(item_type)    
        {
            item_inventory.inventory.add_item(Item { item_type });
        }
    }

    if is_key_just_pressed(&input, KeyCode::Enter) {
        app_state_next_state.set(GameState::Play);
    }
}

fn limit_movement(
    char_position: (f32, f32),
    direction: Vec3
) -> bool {
    let left_border = char_position.0 < MOVE_AREA_POS.0 + CHAR_SIZE / 2.0 && direction.x < 0.0;
    let right_border = char_position.0 > MOVE_AREA_POS.0 + MOVE_AREA_SIZE.0 - CHAR_SIZE / 2.0 && direction.x > 0.0;
    let bottom_border = char_position.1 < MOVE_AREA_POS.1 + CHAR_SIZE / 2.0 && direction.y < 0.0;
    let top_border = char_position.1 > MOVE_AREA_POS.1 + MOVE_AREA_SIZE.1 - CHAR_SIZE / 2.0 && direction.y > 0.0;

    left_border || right_border || bottom_border || top_border
}

pub fn handle_char_movement(
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut char_query: Query<(&mut Transform, &mut Handle<Image>, &mut TextureAtlas), With<Character>>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut movement: ResMut<CharMovement>,
    time: Res<Time>
) -> () {
    if let Ok(
        (mut char_transform, mut image, mut atlas)
    ) = char_query.get_single_mut() {
        let (mut x, mut y): (f32, f32) = (0.0, 0.0);
        let mut direction: Vec3 = Vec3::ZERO;
    
        if is_key_pressed(&input, KeyCode::ArrowLeft) { x = -1.0 }
        if is_key_pressed(&input, KeyCode::ArrowRight) { x = 1.0 }
        if is_key_pressed(&input, KeyCode::ArrowDown) { y = -1.0 }
        if is_key_pressed(&input, KeyCode::ArrowUp) { y = 1.0 }
    
        if x != 0.0 || y != 0.0 {
            direction += Vec3::new(x, y, 0.0);
            direction = direction.normalize();
            movement.set_state(MovementState::Move);
        } else {
            movement.set_state(MovementState::Idle);
        }
    
        if limit_movement(
            (
                char_transform.translation.x,
                char_transform.translation.y
            ), 
            direction
        ) { 
            return; 
        }
    
        if x < 0.0 {
            char_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        } else {
            char_transform.rotation = Quat::default();
        }
    
        char_transform.translation += direction * movement.get_speed() * time.delta_seconds();
    
        set_atlas_layout(
            movement,
            asset_server,
            &mut image, 
            &mut atlas,
            texture_atlas_layouts
        )
    } else {
        return;
    }
    
}
