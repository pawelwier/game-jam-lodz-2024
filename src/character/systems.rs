use bevy::prelude::*;

use crate::{animation::{components::{AnimatedEntity, AnimationIndices, SpriteLayout}, systems::spawn_animated_entity}, game::components::GameBg};

use super::{components::Character, resources::MovementState, CHAR_FRAME_COUNT, CHAR_SIZE};

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
        println!("asd");
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