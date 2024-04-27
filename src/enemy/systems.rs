use bevy::prelude::*;

use crate::game::{MOVE_AREA_MARGIN, MOVE_AREA_SIZE};

use super::{components::{Laser, Ufo}, resources::LaserTimer, LASER_SPEED, UFO_BOTTOM_POS, UFO_LEFT_POS, UFO_POSITIONS, UFO_RIGHT_POS, UFO_TOP_POS};

pub fn tick_laser_timer(
    mut laser_timer: ResMut<LaserTimer>,
    time: Res<Time>
) {
    laser_timer.up_down_timer.tick(time.delta());
    laser_timer.left_right_timer.tick(time.delta());
}

pub fn spawn_ufos(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) -> () {
    for (x, y, ufo_type) in UFO_POSITIONS {
        commands.spawn(
            (
                SpriteBundle {
                    texture: asset_server.load("sprites/ufo.png"),
                    transform: Transform { 
                        translation: Vec3 { x, y, z: 0.0 },
                        scale: Vec3 { x: 0.6, y: 0.6, z: 0.0 },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Ufo {
                    ufo_type
                }
            ),
        );
    }
}

pub fn shoot_lasers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    laser_timer: Res<LaserTimer>,
) -> () {
    // TODO: refactor!!!
    if laser_timer.left_right_timer.finished() {
        let (left_x, left_y) = UFO_LEFT_POS;
        let (right_x, right_y) = UFO_RIGHT_POS;

        commands.spawn(
            (
                SpriteBundle {
                    texture: asset_server.load("sprites/laser.png"),
                    transform: Transform { 
                        translation: Vec3 { x: left_x, y: left_y, z: 0.0 },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Laser {
                    direction_x: rand::random::<f32>(),
                    direction_y: rand::random::<f32>()
                }
            )
        );

        commands.spawn(
            (
                SpriteBundle {
                    texture: asset_server.load("sprites/laser.png"),
                    transform: Transform { 
                        translation: Vec3 { x: right_x, y: right_y, z: 0.0 },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Laser {
                    direction_x: -rand::random::<f32>(),
                    direction_y: -rand::random::<f32>()
                }
            )
        );
    }

    if laser_timer.up_down_timer.finished() {
        let (bottom_x, bottom_y) = UFO_BOTTOM_POS;
        let (top_x, top_y) = UFO_TOP_POS;

        commands.spawn(
            (
                SpriteBundle {
                    texture: asset_server.load("sprites/laser.png"),
                    transform: Transform { 
                        translation: Vec3 { x: top_x, y: top_y, z: 0.0 },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Laser {
                    direction_x: -rand::random::<f32>(),
                    direction_y: -rand::random::<f32>()
                }
            )
        );

        commands.spawn(
            (
                SpriteBundle {
                    texture: asset_server.load("sprites/laser.png"),
                    transform: Transform { 
                        translation: Vec3 { x: bottom_x, y: bottom_y, z: 0.0 },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Laser {
                    direction_x: rand::random::<f32>(),
                    direction_y: rand::random::<f32>()
                }
            )
        );
    }
}

pub fn move_lasers(
    mut commands: Commands,
    mut laser_query: Query<(&mut Transform, Entity, &Laser), With<Laser>>,
    time: Res<Time>
) -> () {
    for (mut transform, entity, laser) in laser_query.iter_mut() {

        let mut direction = Vec3::new(laser.direction_x, laser.direction_y, 0.0);
        direction = direction.normalize();
        transform.translation += direction * LASER_SPEED * time.delta_seconds();

        if transform.translation.y < MOVE_AREA_MARGIN
            || transform.translation.y > MOVE_AREA_MARGIN + MOVE_AREA_SIZE.1 
                || transform.translation.x < MOVE_AREA_MARGIN
                    || transform.translation.x > MOVE_AREA_MARGIN + MOVE_AREA_SIZE.0 {
            commands.entity(entity).despawn();
        }
    } 
}