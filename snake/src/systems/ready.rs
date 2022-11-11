use bevy::{prelude::*, app::AppExit};

use crate::{components::prelude::{Snake, SnakeDirection, Bonus, Spawnable}, resources::{game_state::GameState, border_set::BorderSet}, common::BONUS_DIAMETER};


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
    let mut new_direction: Option<SnakeDirection> = None;

    if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        new_direction = Some(SnakeDirection::Right);
    }
    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::Q]) {
        new_direction = Some(SnakeDirection::Left);
    }
    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::Z]) {
        new_direction = Some(SnakeDirection::Up);
    }
    if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        new_direction = Some(SnakeDirection::Down);
    }

    if let Some(new_direction) = new_direction {
        let mut snake = query.single_mut();
        snake.direction = Some(new_direction);
        game_state.set(GameState::Running).unwrap();
    }
}