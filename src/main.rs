use animation::systems::animate_sprites;
use bevy::{prelude::*, window::WindowResolution};
use character::{resources::CharMovement, systems::spawn_character};
use controller::systems::{handle_add_item, handle_char_movement};
use game::{systems::{draw_game_bg, spawn_camera}, WINDOW_HEIGHT, WINDOW_WIDTH};
use item::{resources::ItemInventory, systems::{draw_inventory_bg, draw_inventory_items}};

mod game;
mod animation;
mod item;
mod controller;
mod character;

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
        .init_resource::<CharMovement>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, draw_inventory_bg)
        // .add_systems(Startup, draw_game_bg.before(spawn_character))
        // .add_systems(Startup, spawn_character)
        .add_systems(Startup, (
            // draw_game_bg,
            spawn_character
        ).chain())
        .add_systems(Update, handle_add_item)
        .add_systems(Update, draw_inventory_items)
        .add_systems(Update, handle_char_movement)
        .add_systems(Update, animate_sprites)
        .run();
}
