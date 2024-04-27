use bevy::{prelude::*, window::PrimaryWindow};

use crate::item::resources::INVENTORY_BG_WIDTH;

use super::{components::GameBg, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) -> () {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..Default::default()
        }
    );
}

pub fn draw_game_bg(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) -> () {
    commands.spawn(
        (
            SpriteBundle {
                texture: asset_server.load("sprites/game_bg.png"),
                transform: Transform { 
                    translation: Vec3 { 
                        x: WINDOW_WIDTH / 2.0 - INVENTORY_BG_WIDTH / 2.0,
                        y: WINDOW_HEIGHT / 2.0, 
                        z: 0.0
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            GameBg {}
        )
    );
}