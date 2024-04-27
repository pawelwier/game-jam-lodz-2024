use bevy::prelude::*;

use crate::{animation::{components::{AnimatedEntity, AnimationIndices, SpriteLayout}, systems::spawn_animated_entity}, enemy::{components::Laser, LASER_SIZE}, game::{components::GameBg, systems::objects_collide, GameState}, item::{components::CharacterItem, resources::CharItemInventory}};

use super::{components::Character, resources::{CharMovement, MovementState}, CHAR_FRAME_COUNT, CHAR_SIZE, CHAR_SPEED};

fn get_char_animated_entity(
    asset_server: Res<AssetServer>,
    path: String,
) -> AnimatedEntity {
    AnimatedEntity {
        texture: asset_server.load(path),
        animation_indices: AnimationIndices { first: 0, last: CHAR_FRAME_COUNT - 1 },
        sprite_layout: SpriteLayout {
            columns: CHAR_FRAME_COUNT,
            rows: 1,
            width: CHAR_SIZE,
            height: CHAR_SIZE
        }
    }
}

pub fn get_char_animation(
    state: &MovementState,
    asset_server: Res<AssetServer>
) -> AnimatedEntity {
    match state {
        MovementState::Idle => 
            get_char_animated_entity(asset_server, "sprites/char_idle.png".to_string()),
        MovementState::Move => 
            get_char_animated_entity(asset_server, "sprites/char_move.png".to_string()),
    }
}

pub fn spawn_character(
    commands: Commands,
    game_bg_query: Query<&Transform, With<GameBg>>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // if let Ok(bg) = game_bg_query.get_single() {
        spawn_animated_entity(
            commands,
            get_char_animation(
                &MovementState::Idle,
                asset_server
            ),
            Vec3 { 
                // x: bg.translation.x / 2.0,
                // y: bg.translation.y / 2.0,
                x: 300.0,
                y: 400.0,
                z: 0.0 
            },
            texture_atlas_layouts,
            Character {}
        );
    // } else { 
        // return;
    // }   
}

pub fn set_character_speed(
    item_inventory: Res<CharItemInventory>,
    mut char_movement: ResMut<CharMovement>
) -> () {
    let item_count = item_inventory.inventory.items.len();

    char_movement.set_speed(CHAR_SPEED - 30.0 * item_count as f32);
}

pub fn check_laser_char_collision(
    laser_query: Query<&Transform, With<Laser>>,
    char_query: Query<&Transform, With<Character>>,
    mut app_state_next_state: ResMut<NextState<GameState>>
) -> () {
    let char_transform = char_query.get_single().unwrap();
    for laser_transform in laser_query.iter() {
        if objects_collide(
            (
                (char_transform.translation.x, char_transform.translation.y),
                (CHAR_SIZE, CHAR_SIZE)
            ),
            (
                (laser_transform.translation.x, laser_transform.translation.y),
                (LASER_SIZE, LASER_SIZE)
            )
        ) {
            app_state_next_state.set(GameState::GameOver);
        }
    }
}