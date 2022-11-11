use bevy::prelude::*;

use crate::resources::game_state::GameState;

use super::prelude::change_system_if_inputs_pressed;

/// Resume the game when the game is paused.
pub fn resume_game(keyboard_input: ResMut<Input<KeyCode>>, game_state: ResMut<State<GameState>>) {
    change_system_if_inputs_pressed(
        GameState::Ready,
        vec![KeyCode::P, KeyCode::Space],
        keyboard_input,
        game_state,
    );
}
