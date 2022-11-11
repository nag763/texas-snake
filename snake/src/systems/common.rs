use bevy::prelude::*;

use crate::{resources::game_state::GameState, components::prelude::Border};

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