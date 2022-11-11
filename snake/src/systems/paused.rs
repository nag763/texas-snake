use bevy::prelude::*;

use crate::resources::game_state::GameState;

/// Resume the game when the game is paused.
pub fn resume_game(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::P]) {
        game_state.push(GameState::Running).unwrap();
        keyboard_input.reset(KeyCode::P);
        keyboard_input.reset(KeyCode::Space);
    }
}