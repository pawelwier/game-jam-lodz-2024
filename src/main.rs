use bevy::{prelude::*, window::WindowResolution};
use controller::systems::handle_add_item;
use item::resources::ItemInventory;

mod item;
mod controller;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 900.0;

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
        .add_systems(Update, handle_add_item)
        .run();
}
