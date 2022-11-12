use bevy::prelude::*;

use crate::{components::prelude::{Border, SnakeDirection}, resources::game_state::GameState};

/// Changes the border visibility when the game is paused or resumed.
pub fn compute_borders_visibility(
    game_state: ResMut<State<GameState>>,
    mut border_query: Query<&mut Visibility, With<Border>>,
) {
    let borders_visibility = game_state.current().are_borders_visible();
    for mut visibility in border_query.iter_mut() {
        visibility.is_visible = borders_visibility;
    }
}

/// Change the current system to the target system if any of the keys
/// passed in inputs are pressed.
pub(crate) fn change_system_if_inputs_pressed(
    target_state: GameState,
    inputs: Vec<KeyCode>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keyboard_input.any_just_pressed(inputs.clone()) {
        game_state.set(target_state).unwrap();
        for input in inputs {
            keyboard_input.reset(input);
        }
    }
}

pub (crate) fn get_direction_from_input(
    keyboard_input: Res<Input<KeyCode>>) -> Option<SnakeDirection> {
    let mut direction : Option<SnakeDirection> = None;
    if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction = Some(SnakeDirection::Right);
    }
    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::Q]) {
        direction = Some(SnakeDirection::Left);
    }
    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::Z]) {
        direction = Some(SnakeDirection::Up);
    }
    if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction = Some(SnakeDirection::Down);
    }
    direction
}
