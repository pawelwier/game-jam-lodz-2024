use animation::systems::animate_sprites;
use bevy::{prelude::*, window::WindowResolution};
use character::{resources::CharMovement, systems::spawn_character};
use controller::systems::{handle_add_item, handle_char_movement};
use game::{systems::{draw_game_bg, spawn_camera}, GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use item::{resources::CharItemInventory, systems::{draw_inventory_bg, draw_inventory_items}};
use point_area::systems::draw_point_areas;

mod game;
mod animation;
mod item;
mod controller;
mod character;
mod point_area;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(
                    WindowPlugin {
                        primary_window: Some(Window {
                            title: "CIEZKOSC BYTU".to_string(),
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
        .init_state::<GameState>()
        .init_resource::<CharItemInventory>()
        .init_resource::<CharMovement>()
        .add_systems(Startup, (
            spawn_camera,
            draw_inventory_bg,
            draw_point_areas,
            spawn_character
        ))
        .add_systems(Update, handle_add_item.run_if(in_state(GameState::LoadInventory)))
        .add_systems(Update, draw_inventory_items)
        .add_systems(Update, (
            handle_char_movement, animate_sprites
        ).run_if(in_state(GameState::Play)))
        .run();
}
