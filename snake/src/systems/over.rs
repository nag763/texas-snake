use bevy::prelude::*;

use crate::resources::{game_state::GameState, score::Score};

/// Restarts the game when it is over.
pub fn restart_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut score: ResMut<Score>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        score.0 = 0;
        game_state.set(GameState::Ready).unwrap();
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        score.0 = 0;
        game_state.set(GameState::Initialized).unwrap();
    }
}
