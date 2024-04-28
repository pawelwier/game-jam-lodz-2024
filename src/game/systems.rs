use bevy::{prelude::*, window::PrimaryWindow};

use crate::{controller::utils::is_key_just_pressed, enemy::resources::LaserTimer, item::resources::INVENTORY_BG_WIDTH, point_area::{resources::ReloadAreasTimer, AREA_POSITIONS}};

use super::{components::GameBg, resources::Score, GameState, MOVE_AREA_MARGIN, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) -> () {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
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
                        x: WINDOW_WIDTH / 2. - INVENTORY_BG_WIDTH / 2. - MOVE_AREA_MARGIN / 2.,
                        y: WINDOW_HEIGHT / 2., 
                        z: 0.
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            GameBg {}
        )
    );
}

pub fn handle_get_ready(
    input: Res<ButtonInput<KeyCode>>,
    mut app_state_next_state: ResMut<NextState<GameState>>
) -> () {
    if is_key_just_pressed(&input, KeyCode::Enter) {
        app_state_next_state.set(GameState::Play);
    }
}

pub fn objects_collide(
    object_one: (
        (f32, f32),
        (f32, f32)
    ),
    object_two: (
        (f32, f32),
        (f32, f32)
    ),
) -> bool {
    let (pos_one, size_one) = object_one;
    let (pos_two, size_two) = object_two;

    (pos_one.0 + size_one.0 / 2. > pos_two.0 - size_two.0 / 2.)
        && (pos_one.0 - size_one.0 / 2. < pos_two.0 + size_two.0 / 2.)
            && (pos_one.1 + size_one.1 / 2. > pos_two.1 - size_two.1 / 2.)
                && (pos_one.1 - size_one.1 / 2. < pos_two.1 + size_two.1 / 2.)
}

pub fn check_game_completed(
    score: Res<Score>,
    mut app_state_next_state: ResMut<NextState<GameState>>
) -> () {
    if score.points == AREA_POSITIONS.len() {
        app_state_next_state.set(GameState::Completed);
    }
}

pub fn reset_timers(
    mut reload_areas_timer: ResMut<ReloadAreasTimer>,
    mut laser_timer: ResMut<LaserTimer>
) -> () {
    reload_areas_timer.timer.reset();
    laser_timer.left_right_timer.reset();
    laser_timer.up_down_timer.reset();
}