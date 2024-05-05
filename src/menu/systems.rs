use bevy::prelude::*;

use crate::{
    character::components::Character, 
    enemy::components::Laser,
    game::{
        resources::Score,
        GameState
    }, 
    item::{
        components::Item,
        resources::CharItemInventory
    }, 
    point_area::{
        components::{
            Area, 
            AreaType
        }, 
        resources::AreaInventories
    }
};

use super::{
    components::{
        BackToMenuButton, FinalMenu, InventoryInfo, MainMenu, PlayAgainButton, PlayButton
    },
    events::FinalMenuClosed,
    helpers::{
        despawn_component, get_endgame_text, get_get_ready_texts, get_menu_button_bundle, get_menu_texts, handle_button_hover, handle_button_none, react_to_endgame_menu_button, spawn_text_bundle
    }
};

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
        spawn_text_bundle(
            parent,
            &asset_server,
            "TABULA RUNNER".to_string(),
            90.,
            Color::DARK_GRAY
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
        ).with_children(|parent: &mut ChildBuilder| {
            spawn_text_bundle(
                parent,
                &asset_server,
                "PLAY".to_string(),
                80.,
                Color::WHITE
            );
        });
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
        for (text, font_size) in get_menu_texts() {
            spawn_text_bundle(
                parent,
                &asset_server,
                text,
                font_size,
                Color::DARK_GRAY
            )
        };
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
        for (text, font_size) in get_get_ready_texts() {
            spawn_text_bundle(
                parent,
                &asset_server,
                text,
                font_size,
                Color::DARK_GRAY
            )
        }
    });    
}

pub fn spawn_endgame_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>
) -> () {
    let endgame_text: String = get_endgame_text(&game_state);
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
        spawn_text_bundle(parent, &asset_server, endgame_text, 100., Color::DARK_GRAY);
        parent.spawn(
            get_menu_button_bundle(PlayAgainButton {})
        ).with_children(|parent| {
            spawn_text_bundle(parent, &asset_server, "PLAY AGAIN".to_string(), 80., Color::WHITE);
        });
        parent.spawn(
            get_menu_button_bundle(BackToMenuButton {})
        ).with_children(|parent| {
            spawn_text_bundle(parent, &asset_server, "BACK TO MENU".to_string(), 80., Color::WHITE)
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
            Interaction::Hovered => { handle_button_hover(&mut bg_color) },
            Interaction::None => { handle_button_none(&mut bg_color) }
        }
    }
}

pub fn react_to_restart_button(
    mut commands: Commands,
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayAgainButton>)>,
    mut final_menu_event_writer: EventWriter<FinalMenuClosed>,
    final_menu_query: Query<Entity, With<FinalMenu>>
) {
    react_to_endgame_menu_button(
        &mut commands,
        &mut button_query,
        &mut final_menu_event_writer,
        &final_menu_query,
        GameState::LoadInventory
    );
}

pub fn react_to_back_to_menu_button(
    mut commands: Commands,
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<BackToMenuButton>)>,
    mut final_menu_event_writer: EventWriter<FinalMenuClosed>,
    final_menu_query: Query<Entity, With<FinalMenu>>
) {
    react_to_endgame_menu_button(
        &mut commands,
        &mut button_query,
        &mut final_menu_event_writer,
        &final_menu_query,
        GameState::MainMenu
    );
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    despawn_component(&mut commands, main_menu_query)
}

pub fn despawn_inventory_info(
    mut commands: Commands,
    inventory_info_query: Query<Entity, With<InventoryInfo>>
) {
    despawn_component(&mut commands, inventory_info_query)
}

pub fn despawn_get_ready_info(
    mut commands: Commands,
    get_ready_info_query: Query<Entity, With<InventoryInfo>>
) {
    despawn_component(&mut commands, get_ready_info_query);
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
    for event in final_menu_event_reader.read() {
        for entity in object_query.iter_mut() {
            commands.entity(entity).despawn();
        }
    
        char_inventory.inventory.clear_items();
    
        score.points = 0;
    
        for i in 0..area_inventories.inventories.len() {
            area_inventories.inventories[i].1.items = Vec::<Item>::new();
            area_inventories.inventories[i].2 = AreaType::Empty;
        }

        app_state_next_state.set(event.new_state);

    }
}