use bevy::prelude::*;

use crate::game::GameState;

use super::{components::{FinalMenu, InventoryInfo, InventoryReadyButton, MainMenu, PlayButton}, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR};

// TODO: add some order, refactor

fn draw_text(
    asset_server: &AssetServer,
    text: String,
    font_size: f32
) -> TextSection {
    TextSection::new(
        text,
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            color: Color::SILVER,
            font_size,
            ..default()
        },
    )
}

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) -> () {
    commands.spawn(
        (
            NodeBundle {
                background_color: BackgroundColor(Color::AZURE),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    row_gap: Val::Px(50.),
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenu {}
        )
    )
    .with_children(|parent| {
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        draw_text(&asset_server, "PLAY PLAY PLAY PLAY".to_string(), 100.),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            }
        );
        parent.spawn(
            (
                ButtonBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(400.),
                        height: Val::Px(100.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                PlayButton {}
            )
        ).with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        draw_text(&asset_server, "Play".to_string(), 80.),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            });
        });
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        draw_text(&asset_server, "CHOOSE DIFFICULTY".to_string(), 100.),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            }
        );
    });
}

pub fn spawn_inventory_info(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) -> () {
    commands.spawn(
        (
            NodeBundle {
                background_color: BackgroundColor(Color::AZURE),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(80.),
                    height: Val::Percent(100.),
                    padding: UiRect {
                        left: Val::Percent(3.),
                        right: Val::Percent(4.),
                        top: Val::Percent(3.),
                        bottom: Val::Percent(3.)
                    },
                    row_gap: Val::Px(50.),
                    ..Default::default()
                },
                ..Default::default()
            },
            InventoryInfo {}
        )
    )
    .with_children(|parent| {
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        draw_text(&asset_server, "Let's make your tabula rasa not so rasa any more.".to_string(), 50.),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            }
        );
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        draw_text(&asset_server, "Press 1 2 3 to fill in your inventory items.".to_string(), 50.),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            }
        );
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        draw_text(&asset_server, "The more items you have the more fields you can mark.".to_string(), 50.),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            }
        );
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        draw_text(&asset_server, "But the more items you have the slower you run.".to_string(), 50.),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            }
        );
    });    
}

pub fn spawn_get_ready_info(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) -> () {
    commands.spawn(
        (
            NodeBundle {
                background_color: BackgroundColor(Color::AZURE),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(30.),
                    row_gap: Val::Px(50.),
                    ..Default::default()
                },
                ..Default::default()
            },
            InventoryInfo {}
        )
    )
    .with_children(|parent| {
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        draw_text(&asset_server, "Press Enter when ready. Good luck!".to_string(), 70.),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            }
        );
    });    
}

pub fn spawn_restart_menu(

) -> () {
    
}

pub fn react_to_play_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut app_state_next_state: ResMut<NextState<GameState>>
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => { app_state_next_state.set(GameState::LoadInventory); },
            Interaction::Hovered => { *bg_color = HOVERED_BUTTON_COLOR.into(); },
            Interaction::None => { *bg_color = NORMAL_BUTTON_COLOR.into(); }
        }
    }
}

pub fn react_to_restart_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<InventoryReadyButton>)>,
    mut app_state_next_state: ResMut<NextState<GameState>>
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => { app_state_next_state.set(GameState::LoadInventory); },
            Interaction::Hovered => { *bg_color = HOVERED_BUTTON_COLOR.into(); },
            Interaction::None => { *bg_color = NORMAL_BUTTON_COLOR.into(); }
        }
    }
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(menu_entity) = main_menu_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

pub fn despawn_inventory_info(
    mut commands: Commands,
    inventory_info_query: Query<Entity, With<InventoryInfo>>
) {
    if let Ok(menu_entity) = inventory_info_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

pub fn despawn_final_menu(
    mut commands: Commands,
    final_menu_query: Query<Entity, With<FinalMenu>>
) {
    if let Ok(menu_entity) = final_menu_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}