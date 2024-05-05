use bevy::prelude::*;

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

pub fn draw_text(
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