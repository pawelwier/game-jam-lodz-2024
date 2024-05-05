use bevy::prelude::*;

use crate::{character::components::Character, enemy::components::Laser, game::{resources::Score, GameState}, item::{components::Item, resources::CharItemInventory}, point_area::{components::{Area, AreaType}, resources::AreaInventories}};

use super::{components::{BackToMenuButton, /*ExitButton,*/ FinalMenu, InventoryInfo, MainMenu, PlayButton}, events::FinalMenuClosed, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR};

// TODO: add some order, refactor

fn draw_text(
    asset_server: &AssetServer,
    text: String,
    font_size: f32,
    color: Color
) -> TextSection {
    TextSection::new(
        text,
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            color,
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
                background_color: BackgroundColor(Color::WHITE),
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
                        draw_text(&asset_server, "TABULA RUNNER".to_string(), 90., Color::DARK_GRAY),
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
                        draw_text(&asset_server, "PLAY".to_string(), 80., Color::WHITE),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            });
        });
        // parent.spawn(
        //     (
        //         ButtonBundle {
        //             style: Style {
        //                 flex_direction: FlexDirection::Row,
        //                 justify_content: JustifyContent::Center,
        //                 align_items: AlignItems::Center,
        //                 width: Val::Px(400.),
        //                 height: Val::Px(100.),
        //                 ..Default::default()
        //             },
        //             ..Default::default()
        //         },
        //         ExitButton {}
        //     )
        // ).with_children(|parent| {
        //     parent.spawn(TextBundle {
        //         text: Text {
        //             sections: vec![
        //                 draw_text(&asset_server, "EXIT".to_string(), 80., Color::WHITE),
        //             ],
        //             justify: JustifyText::Center,
        //             ..default()
        //         },
        //         ..Default::default()
        //     });
        // });
        // parent.spawn(
        //     TextBundle {
        //         text: Text {
        //             sections: vec![
        //                 draw_text(&asset_server, "CHOOSE DIFFICULTY".to_string(), 100., Color::DARK_GRAY),
        //             ],
        //             justify: JustifyText::Center,
        //             ..default()
        //         },
        //         ..Default::default()
        //     }
        // );
    });
}

pub fn spawn_inventory_info(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) -> () {
    commands.spawn(
        (
            NodeBundle {
                background_color: BackgroundColor(Color::WHITE),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(75.),
                    height: Val::Percent(100.),
                    padding: UiRect {
                        left: Val::Percent(2.),
                        right: Val::Percent(2.),
                        top: Val::Percent(2.),
                        bottom: Val::Percent(2.)
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
                        draw_text(&asset_server, "What you see on the right is your tabula rasa.".to_string(), 35., Color::DARK_GRAY),
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
                        draw_text(&asset_server, "Let's make your tabula rasa not so rasa any more.".to_string(), 35., Color::DARK_GRAY),
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
                        draw_text(&asset_server, "The more items you have the more fields you can mark.".to_string(), 35., Color::DARK_GRAY),
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
                        draw_text(&asset_server, "But the more items you have the slower you run.".to_string(), 35., Color::DARK_GRAY),
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
                        draw_text(&asset_server, "You mark a field by running on it. You need at least as many items of each kind as there are on the field.".to_string(), 35., Color::DARK_GRAY),
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
                        draw_text(&asset_server, "Press 1 2 3 to fill in your inventory items (max. 3 each).".to_string(), 50., Color::DARK_GRAY),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            }
        );
        parent.spawn(
            ImageBundle {
                image: UiImage { 
                    texture: asset_server.load("sprites/help_info.png"),
                    ..Default::default()
                },
                z_index: ZIndex::Global(90),
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
                background_color: BackgroundColor(Color::WHITE),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(20.),
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
                        draw_text(&asset_server, "Use arrows to move. Watch out for the lasers.".to_string(), 50., Color::DARK_GRAY),
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
                        draw_text(&asset_server, "Press ENTER when ready. Good luck!".to_string(), 50., Color::DARK_GRAY),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            }
        );
    });    
}

pub fn spawn_endgame_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>
) -> () {
    let endgame_text: String = if *game_state.get() == GameState::GameOver {
        "GAME OVER".to_string()
    } else {
        "CONGRATULATIONS".to_string()
    };
    commands.spawn(
        (
            NodeBundle {
                background_color: BackgroundColor(Color::WHITE),
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
            FinalMenu {}
        )
    )
    .with_children(|parent| {
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        draw_text(&asset_server, endgame_text, 100., Color::DARK_GRAY),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            }
        );
        // parent.spawn(
        //     (
        //         ButtonBundle {
        //             style: Style {
        //                 flex_direction: FlexDirection::Row,
        //                 justify_content: JustifyContent::Center,
        //                 align_items: AlignItems::Center,
        //                 width: Val::Px(600.),
        //                 height: Val::Px(100.),
        //                 ..Default::default()
        //             },
        //             ..Default::default()
        //         },
        //         PlayAgainButton {}
        //     )
        // ).with_children(|parent| {
        //     parent.spawn(TextBundle {
        //         text: Text {
        //             sections: vec![
        //                 draw_text(&asset_server, "PLAY AGAIN".to_string(), 80., Color::WHITE),
        //             ],
        //             justify: JustifyText::Center,
        //             ..default()
        //         },
        //         ..Default::default()
        //     });
        // });
        parent.spawn(
            (
                ButtonBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(600.),
                        height: Val::Px(100.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                BackToMenuButton {}
            )
        ).with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        draw_text(&asset_server, "BACK TO MENU".to_string(), 80., Color::WHITE),
                    ],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..Default::default()
            });
        });
    });
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

// pub fn react_to_exit_button(
//     mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ExitButton>)>,
//     mut app_exit_events: ResMut<Events<AppExit>>
// ) {
//     if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
//         match *interaction {
//             Interaction::Pressed => { app_exit_events.send(AppExit); },
//             Interaction::Hovered => { *bg_color = HOVERED_BUTTON_COLOR.into(); },
//             Interaction::None => { *bg_color = NORMAL_BUTTON_COLOR.into(); }
//         }
//     }
// }

// pub fn react_to_restart_button(
//     mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayAgainButton>)>,
//     mut app_state_next_state: ResMut<NextState<GameState>>,
//     mut final_menu_event_writer: EventWriter<FinalMenuClosed>
// ) {
//     if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
//         match *interaction {
//             Interaction::Pressed => { 
//                 app_state_next_state.set(GameState::LoadInventory); 
//                 final_menu_event_writer.send(FinalMenuClosed {});
//             },
//             Interaction::Hovered => { *bg_color = HOVERED_BUTTON_COLOR.into(); },
//             Interaction::None => { *bg_color = NORMAL_BUTTON_COLOR.into(); }
//         }
//     }
// }

pub fn react_to_back_to_menu_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<BackToMenuButton>)>,
    mut final_menu_event_writer: EventWriter<FinalMenuClosed>,
    mut commands: Commands,
    final_menu_query: Query<Entity, With<FinalMenu>>
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(menu_entity) = final_menu_query.get_single() {
                    commands.entity(menu_entity).despawn_recursive();
                    final_menu_event_writer.send(FinalMenuClosed {});
                }
            },
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

pub fn despawn_get_ready_info(
    mut commands: Commands,
    get_ready_info_query: Query<Entity, With<InventoryInfo>>
) {
    if let Ok(menu_entity) = get_ready_info_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

pub fn check_despawn_final_menu(
    mut commands: Commands,
    mut final_menu_event_reader: EventReader<FinalMenuClosed>,
    mut object_query: Query<Entity, Or<(
        With<Area>, With<Character>, With<Item>, With<Laser>
    )>>,
    mut char_inventory: ResMut<CharItemInventory>,
    mut area_inventories: ResMut<AreaInventories>,
    mut score: ResMut<Score>,
    mut app_state_next_state: ResMut<NextState<GameState>>
) {
    for _ in final_menu_event_reader.read() {
        for entity in object_query.iter_mut() {
            commands.entity(entity).despawn();
        }
    
        char_inventory.inventory.clear_items();
    
        score.points = 0;
    
        for i in 0..area_inventories.inventories.len() {
            area_inventories.inventories[i].1.items = Vec::<Item>::new();
            area_inventories.inventories[i].2 = AreaType::Empty;
        }

        app_state_next_state.set(GameState::MainMenu);

    }
}