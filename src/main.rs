use bevy::{prelude::*, window::{PrimaryWindow, WindowResolution}};
use controller::systems::handle_add_item;
use item::{resources::{ItemInventory, INVENTORY_BG_WIDTH}, systems::{draw_inventory_bg, draw_inventory_items}};

mod item;
mod controller;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 900.0;

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
        )
    );
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(
                    WindowPlugin {
                        primary_window: Some(Window {
                            title: "GAME JAM LODZ".to_string(),
                            resizable: false,
                            resolution: WindowResolution::new(
                                WINDOW_WIDTH,
                                WINDOW_HEIGHT 
                            ),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                )
        )
        .init_resource::<ItemInventory>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, draw_inventory_bg)
        .add_systems(Startup, draw_game_bg)
        .add_systems(Update, handle_add_item)
        .add_systems(Update, draw_inventory_items)
        .run();
}
