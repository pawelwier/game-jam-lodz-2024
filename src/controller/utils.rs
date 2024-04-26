use bevy::prelude::*;

pub fn is_key_pressed(
    input: &Res<ButtonInput<KeyCode>>,
    code: KeyCode
) -> bool {
    input.pressed(code)
}

pub fn is_key_just_pressed(
    input: &Res<ButtonInput<KeyCode>>,
    code: KeyCode
) -> bool {
    input.just_pressed(code)
}