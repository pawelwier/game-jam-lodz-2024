use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::GameState;

use super::{components::FinalMenu, events::FinalMenuClosed, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR};

pub fn get_menu_texts() -> [(String, f32); 6] {
    [
        ("What you see on the right is your tabula rasa.".to_string(), 35.),
        ("Let's make your tabula rasa not so rasa any more.".to_string(), 35.),
        ("The more items you have the more fields you can mark.".to_string(), 35.),
        ("But the more items you have the slower you run.".to_string(), 35.),
        ("You mark a field by running on it. You need at least as many items of each kind as there are on the field.".to_string(), 35.),
        ("Press 1 2 3 to fill in your inventory items (max. 3 each).".to_string(), 50.),
    ]
}

pub fn get_get_ready_texts() -> [(String, f32); 2] {
    [
        ("Use arrows to move. Watch out for the lasers.".to_string(), 50.),
        ("Press ENTER when ready. Good luck!".to_string(), 50.)
    ]
}

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

pub fn spawn_text_bundle(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    text: String,
    font_size: f32,
    color: Color
) -> () {
    parent.spawn(
        TextBundle {
            text: Text {
                sections: vec![
                    draw_text(&asset_server, text, font_size, color),
                ],
                justify: JustifyText::Center,
                ..default()
            },
            ..Default::default()
        }
    );
}

pub fn despawn_component(
    commands: &mut Commands,
    query: Query<Entity, With<impl Component>>
) -> () {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn get_menu_button_bundle(
    component: impl Component
) -> (ButtonBundle, impl Component) {
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
            component
        )
}

pub fn handle_button_hover(
    color: &mut BackgroundColor
) -> () {
    *color = HOVERED_BUTTON_COLOR.into();
}

pub fn handle_button_none(
    color: &mut BackgroundColor
) -> () {
    *color = NORMAL_BUTTON_COLOR.into();
}

pub fn react_to_endgame_menu_button(
    commands: &mut Commands,
    button_query: &mut Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<impl Component>)>,
    final_menu_event_writer: &mut EventWriter<FinalMenuClosed>,
    final_menu_query: &Query<Entity, With<FinalMenu>>,
    new_state: GameState
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => { 
                if let Ok(menu_entity) = final_menu_query.get_single() {
                    commands.entity(menu_entity).despawn_recursive();
                    final_menu_event_writer.send(FinalMenuClosed {
                        new_state
                    });
                }
            },
            Interaction::Hovered => { handle_button_hover(&mut bg_color) },
            Interaction::None => { handle_button_none(&mut bg_color) }
        }
    }
}

pub fn get_endgame_text(
    game_state: &GameState
) -> String {
    let game_state_text_map: HashMap<GameState, &str> = HashMap::from([
        (GameState::GameOver, "GAME OVER"),
        (GameState::Completed, "CONGRATULATIONS")
    ]);

    game_state_text_map.get(&game_state).unwrap_or(&"").to_string()
}