use bevy::{app::AppExit, prelude::*};

use crate::{
    common::BONUS_DIAMETER,
    components::prelude::{Bonus, Snake, Spawnable},
    resources::{border_set::BorderSet, game_state::GameState},
};

use super::prelude::get_direction_from_input;

/// Init the game components, allowing the user to interact with the system.
pub fn init_game_components(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    border_set: Res<Option<BorderSet>>,
    mut exit: EventWriter<AppExit>,
) {
    if let Some(border_set) = *border_set {
        let snake_initial_position = border_set.get_snake_initial_position();
        let bonus_initial_position = border_set.compute_random_bonus_position();

        // Spawn snake
        Snake::default().spawn(
            Transform::default().with_translation(snake_initial_position),
            &mut commands,
            &mut materials,
            &mut meshes,
        );
        // The first bonus
        Bonus::default().spawn(
            Transform::default()
                .with_scale(Vec3::splat(BONUS_DIAMETER))
                .with_translation(bonus_initial_position),
            &mut commands,
            &mut materials,
            &mut meshes,
        );
        border_set.spawn_borders(commands, materials, meshes);
    } else {
        eprintln!("Unreachable");
        exit.send(AppExit);
    }
}

/// Set the first direction of the snake, when the game is initiallized.
pub fn set_first_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<&mut Snake>,
) {

    if let Some(new_direction) = get_direction_from_input(keyboard_input) {
        let mut snake = query.single_mut();
        snake.direction = Some(new_direction);
        game_state.set(GameState::Running).unwrap();
    }
}
