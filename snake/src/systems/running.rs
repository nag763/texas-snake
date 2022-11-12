use bevy::prelude::*;

use crate::{
    common::*,
    components::prelude::{Bonus, Collider, Queue, Snake},
    resources::game_state::GameState,
    CollisionEvent, ExtraBonusTimer,
};

use super::prelude::{change_system_if_inputs_pressed, get_direction_from_input};

/// The movement of snake per TIME_STEP applied to the ball.
pub fn move_snake(mut query: Query<(&mut Transform, &mut Snake)>) {
    let (mut transform, mut snake) = query.single_mut();
    if let Some(direction) = snake.direction {
        let translation_diff = direction.into_translation() * SNAKE_SPEED_FACTOR;
        snake.last_position = transform.translation;
        let mut new_translation = transform.translation + translation_diff;
        // Upper or lower component translation when there is no border
        if MAX_SCREEN_WIDTH < f32::abs(new_translation.x) {
            new_translation.x = -MAX_SCREEN_WIDTH * new_translation.x.signum();
        }
        // Upper or lower component translation when there is no border
        if MAX_SCREEN_HEIGHT < f32::abs(new_translation.y) {
            new_translation.y = -MAX_SCREEN_HEIGHT * new_translation.y.signum();
        }
        transform.translation = new_translation;
    }
}

/// Moves the queue of the snake, where n+1 position = n position and 0 = snake's last position
pub fn move_queue(mut query: Query<&mut Transform, With<Queue>>, snake: Query<&Snake>) {
    let snake = snake.single();
    let mut last_position = snake.last_position;
    for mut transform in query.iter_mut() {
        std::mem::swap(&mut transform.translation, &mut last_position)
    }
}

/// Check whether the snake has collided anything, a bonus or a border.
pub fn check_collisions(
    snake: Query<&Transform, With<Snake>>,
    colliders: Query<(&Transform, Option<&Bonus>), With<Collider>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    let snake_position = snake.single().translation;
    for (collider, maybe_bonus) in colliders.iter() {
        let collider_dimensions = Vec2::new(collider.scale.x, collider.scale.y);
        let collide = bevy::sprite::collide_aabb::collide(
            snake_position,
            SNAKE_DIMENSIONS,
            collider.translation,
            collider_dimensions,
        );
        if collide.is_some() {
            if let Some(bonus) = maybe_bonus {
                collision_event_writer.send(CollisionEvent::Bonus(bonus.get_points()));
            } else {
                // If the collider isn't a bonus, it is a border.
                collision_event_writer.send_default();
            }
        }
    }
}
/// Timeouts the extra bonus if it is on the screen.
pub fn extra_bonus_timeout(
    mut commands: Commands,
    bonus_query: Query<(Entity, &Handle<ColorMaterial>, &Bonus)>,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut extra_bonus_timer: ResMut<ExtraBonusTimer>,
) {
    // We rather iter here, since we do not know whether an extra bonus is on the screen or
    // not. However, only one extra bonus should be on the screen.
    for (entity, material, bonus) in bonus_query.iter() {
        match bonus {
            Bonus::Normal => (),
            Bonus::ExtraBonus => {
                extra_bonus_timer.tick(time.delta());
                let elapsed_secs = extra_bonus_timer.elapsed_secs();
                // While the extra bonus is on the screen, we fade it out.
                if elapsed_secs < TIME_FOR_BONUS {
                    let new_alpha = 1f32 - elapsed_secs / TIME_FOR_BONUS;
                    let mut color_mat = materials.get_mut(material).unwrap();
                    color_mat.color = Color::Rgba {
                        red: EXTRA_BONUS_RGB.0,
                        green: EXTRA_BONUS_RGB.1,
                        blue: EXTRA_BONUS_RGB.2,
                        alpha: new_alpha,
                    };
                // If the time limit has been reached, we despawn the extra bonus.
                } else {
                    commands.entity(entity).despawn();
                    extra_bonus_timer.reset();
                }
            }
        }
    }
}

/// Enter in pause when the game is running.
pub fn enter_pause(keyboard_input: ResMut<Input<KeyCode>>, game_state: ResMut<State<GameState>>) {
    change_system_if_inputs_pressed(
        GameState::Paused,
        vec![KeyCode::P, KeyCode::Space],
        keyboard_input,
        game_state,
    );
}

/// Handle a snake direction change on input.
pub fn handle_snake_direction_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Snake>,
) {
    if let Some(new_direction) = get_direction_from_input(keyboard_input) {
        let mut snake = query.single_mut();
        if !snake.direction.unwrap().conflicts_with(new_direction) {
            snake.direction = Some(new_direction);
        }
    }
}
