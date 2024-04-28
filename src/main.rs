use animation::systems::animate_sprites;
use bevy::{prelude::*, window::WindowResolution};
use character::{resources::CharMovement, systems::{check_laser_char_collision, set_character_speed, spawn_character}};
use controller::systems::{handle_add_item, handle_char_movement};
use enemy::{resources::LaserTimer, systems::{move_lasers, shoot_lasers, spawn_ufos, tick_laser_timer}};
use game::{resources::Score, systems::{check_game_completed, draw_game_bg, handle_get_ready, spawn_camera}, GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use item::{resources::CharItemInventory, systems::{draw_char_inventory_items, draw_inventory_bg}};
use menu::systems::{despawn_inventory_info, despawn_main_menu, react_to_play_button, react_to_restart_button, spawn_inventory_info, spawn_main_menu};
use point_area::{
    events::AreaCaptured, 
    resources::{AreaInventories, ReloadAreasTimer}, 
    systems::{
        add_random_area_items, check_step_on_area, draw_point_areas, handle_area_captured, reload_areas_over_time, tick_reload_areas_timer
    }
};

mod game;
mod animation;
mod item;
mod controller;
mod character;
mod point_area;
mod enemy;
mod menu;

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
        .init_resource::<AreaInventories>()
        .init_resource::<Score>()
        .init_resource::<ReloadAreasTimer>()
        .init_resource::<LaserTimer>()
        .add_event::<AreaCaptured>()
        .add_systems(Startup, (
            spawn_camera,
            draw_inventory_bg,
            draw_game_bg
        ))
        .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
        .add_systems(OnEnter(GameState::LoadInventory), (
            despawn_main_menu, spawn_inventory_info
        ))
        .add_systems(Update, react_to_play_button.run_if(in_state(GameState::MainMenu)))

        .add_systems(Update, (
                handle_add_item, draw_char_inventory_items
            ).run_if(in_state(GameState::LoadInventory))
        )
        .add_systems(OnEnter(GameState::GetReady), (
            draw_point_areas, handle_get_ready, despawn_inventory_info
        ))
        .add_systems(Update, handle_get_ready.run_if(in_state(GameState::GetReady)))
        .add_systems(OnEnter(GameState::Play), (
            add_random_area_items,
            set_character_speed,
            spawn_character,
            spawn_ufos
        ))
        .add_systems(Update, (
            handle_char_movement,
            animate_sprites,
            check_step_on_area,
            handle_area_captured,
            tick_reload_areas_timer,
            reload_areas_over_time,
            tick_laser_timer,
            shoot_lasers,
            move_lasers,
            check_laser_char_collision,
            check_game_completed
        ).run_if(in_state(GameState::Play)))
        .add_systems(Update, react_to_restart_button.run_if(in_state(GameState::GameOver)))
        .run();
}
