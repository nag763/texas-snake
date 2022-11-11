pub mod common;
mod components;
pub mod resources;

use common::*;
use components::prelude::*;
use resources::prelude::*;

use rand::Rng;

use bevy::{app::AppExit, prelude::*, sprite::collide_aabb::collide, time::Stopwatch};

#[derive(Debug, Component)]
struct UserText;

/// The event following a conflict of position between the snake and a collider.
#[derive(Default)]
enum CollisionEvent {
    #[default]
    Border,
    Bonus(u32),
}

#[derive(Clone, Deref, DerefMut, Default)]
struct ExtraBonusTimer(Stopwatch);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Score>()
        .init_resource::<Option<BorderSet>>()
        .init_resource::<ExtraBonusTimer>()
        .init_resource::<AppFont>()
        .add_state::<GameState>(GameState::default())
        .add_event::<CollisionEvent>()
        .add_startup_system(setup)
        .add_startup_system(load_assets)
        .add_startup_system(window_resize_system)
        .add_system(update_text)
        .add_system_set(
            SystemSet::on_enter(GameState::Initialized).with_system(spawn_border_set_buttons),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Initialized).with_system(border_set_choose_system),
        )
        .add_system_set(SystemSet::on_exit(GameState::Initialized).with_system(delete_buttons))
        .add_system_set(SystemSet::on_enter(GameState::Ready).with_system(init_game_components))
        .add_system_set(SystemSet::on_update(GameState::Ready).with_system(set_first_direction))
        .add_system_set(
            SystemSet::on_enter(GameState::Running).with_system(compute_borders_visibility),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_system(enter_pause)
                .with_system(check_collisions)
                .with_system(handle_snake_direction_input)
                .with_system(extra_bonus_timeout.before(handle_snake_direction_input))
                .with_system(move_snake.before(handle_snake_direction_input))
                .with_system(move_queue.before(move_snake))
                .with_system(
                    collision_handler
                        .before(handle_snake_direction_input)
                        .after(check_collisions),
                ),
        )

        .add_system_set(
            SystemSet::on_enter(GameState::Paused).with_system(compute_borders_visibility),
        )
        .add_system_set(SystemSet::on_update(GameState::Paused).with_system(resume_game))
        .add_system_set(SystemSet::on_update(GameState::Over).with_system(restart_game))
        .run();
}

fn load_assets(asset_server: Res<AssetServer>, mut app_font: ResMut<AppFont>) {
    let font: Handle<Font> = asset_server.load(FONT_ASSET_NAME);
    **app_font = Some(font);
}

fn setup(mut commands: Commands) {
    // The camera
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(
            TextBundle::from_sections([TextSection::default()])
                .with_style(Style::default())
                .with_text_alignment(TextAlignment::CENTER),
        )
        .insert(UserText);
}

fn spawn_border_set_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    for border_set_variant in BorderSet::iterator() {
        commands
            .spawn()
            .insert_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    border_set_variant.to_string(),
                    TextStyle {
                        font: asset_server.load(FONT_ASSET_NAME),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ));
            })
            .insert(border_set_variant);
    }
}

fn init_game_components(
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

/// Resizes the window at startup.
fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(APP_TITLE.into());
    window.set_resolution(SCREEN_WIDTH, SCREEN_HEIGHT);
}

fn border_set_choose_system(
    mut button_query: Query<(&Interaction, &mut UiColor, &BorderSet)>,
    mut game_state: ResMut<State<GameState>>,
    mut border_set: ResMut<Option<BorderSet>>,
) {
    for (interaction, mut color, button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *border_set = Some(*button);
                game_state.set(GameState::Ready).unwrap();
                break;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn delete_buttons(mut commands: Commands, mut button_query: Query<Entity, With<BorderSet>>) {
    for button_entity in button_query.iter_mut() {
        commands.entity(button_entity).despawn_recursive();
    }
}

/// Updates the displayed score on the screen.                  
fn update_text(
    game_state: Res<State<GameState>>,
    app_font: Res<AppFont>,
    score: Res<Score>,
    mut query: Query<(&mut Text, &mut Style), With<UserText>>,
    mut exit: EventWriter<AppExit>,
) {
    let (mut text, mut style) = query.single_mut();
    if let Some(font) = &**app_font {
        *text = game_state.current().get_score_text(*score, font.clone());
        *style = game_state.current().get_score_style();
    } else {
        eprintln!("Assets were not correctly loaded on startup");
        exit.send(AppExit);
    }
}

/// The movement of snake per TIME_STEP applied to the ball.
fn move_snake(mut query: Query<(&mut Transform, &mut Snake)>) {
    let (mut transform, mut snake) = query.single_mut();
    if let Some(direction) = snake.direction {
        let translation_diff = direction.into_translation() * SNAKE_SPEED_FACTOR;
        snake.last_position = transform.translation;
        let mut new_translation = transform.translation + translation_diff;
        if MAX_SCREEN_WIDTH < f32::abs(new_translation.x) {
            new_translation.x = -MAX_SCREEN_WIDTH * new_translation.x.signum();
        }
        if MAX_SCREEN_HEIGHT < f32::abs(new_translation.y) {
            new_translation.y = -MAX_SCREEN_HEIGHT * new_translation.y.signum();
        }
        transform.translation = new_translation;
    }
}

fn move_queue(mut query: Query<&mut Transform, With<Queue>>, snake: Query<&Snake>) {
    let snake = snake.single();
    let mut last_position = snake.last_position;
    for mut transform in query.iter_mut() {
        std::mem::swap(&mut transform.translation, &mut last_position)
    }
}

fn check_collisions(
    snake: Query<&Transform, With<Snake>>,
    colliders: Query<(&Transform, Option<&Bonus>), With<Collider>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    let snake_position = snake.single().translation;
    for (collider, maybe_bonus) in colliders.iter() {
        let collider_dimensions = Vec2::new(collider.scale.x, collider.scale.y);
        let collide = collide(
            snake_position,
            SNAKE_DIMENSIONS,
            collider.translation,
            collider_dimensions,
        );
        if collide.is_some() {
            if let Some(bonus) = maybe_bonus {
                let points = match bonus {
                    Bonus::ExtraBonus => 5,
                    Bonus::Normal => 1,
                };
                collision_event_writer.send(CollisionEvent::Bonus(points));
            } else {
                collision_event_writer.send_default();
            }
        }
    }
}

fn extra_bonus_timeout(
    mut commands: Commands,
    bonus_query: Query<(Entity, &Handle<ColorMaterial>, &Bonus)>,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut extra_bonus_timer: ResMut<ExtraBonusTimer>,
) {
    for (entity, material, bonus) in bonus_query.iter() {
        match bonus {
            Bonus::Normal => (),
            Bonus::ExtraBonus => {
                extra_bonus_timer.tick(time.delta());
                let elapsed_secs = extra_bonus_timer.elapsed_secs();
                if elapsed_secs < TIME_FOR_BONUS {
                    let new_alpha = 1f32 - elapsed_secs / TIME_FOR_BONUS;
                    let mut color_mat = materials.get_mut(material).unwrap();
                    color_mat.color = Color::Rgba {
                        red: EXTRA_BONUS_RGB.0,
                        green: EXTRA_BONUS_RGB.1,
                        blue: EXTRA_BONUS_RGB.2,
                        alpha: new_alpha,
                    };
                } else {
                    commands.entity(entity).despawn();
                    extra_bonus_timer.reset();
                }
            }
        }
    }
}

fn collision_handler(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut bonus: Query<(&mut Transform, Entity, &Bonus)>,
    mut score: ResMut<Score>,
    mut game_state: ResMut<State<GameState>>,
    mut border_query: Query<Entity, With<Border>>,
    mut extra_bonus_timer: ResMut<ExtraBonusTimer>,
    border_set: Res<Option<BorderSet>>,
    snake: Query<Entity, With<Snake>>,
    queue: Query<Entity, With<Queue>>,
) {
    if let Some(event) = collision_event_reader.iter().next() {
        match event {
            CollisionEvent::Bonus(points) => {
                for _ in 0..*points {
                    // spawn a queue
                    Queue::default().spawn(
                        Transform::default().with_translation(Vec3::new(
                            SCREEN_WIDTH,
                            SCREEN_HEIGHT,
                            0f32,
                        )),
                        &mut commands,
                        &mut materials,
                        &mut meshes,
                    );
                }
                score.0 += points;
                let mut extra_bonus_exists: bool = false;
                for (mut bonus_position, bonus_entity, bonus) in bonus.iter_mut() {
                    match bonus {
                        Bonus::Normal if points == &1u32 => {
                            bonus_position.translation =
                                border_set.unwrap().compute_random_bonus_position();
                        }
                        Bonus::ExtraBonus if points == &5u32 => {
                            extra_bonus_timer.reset();
                            commands.entity(bonus_entity).despawn();
                        }
                        Bonus::ExtraBonus if points == &1u32 => {
                            extra_bonus_exists = true;
                        }
                        _ => (),
                    }
                }
                let mut rng = rand::thread_rng();
                if points == &1 && !extra_bonus_exists && rng.gen_bool(CHANCE_OF_EXTRA_BONUS) {
                    let extra_bonus_position = border_set.unwrap().compute_random_bonus_position();
                    Bonus::ExtraBonus.spawn(
                        Transform::default()
                            .with_scale(Vec3::splat(BONUS_DIAMETER))
                            .with_translation(extra_bonus_position),
                        &mut commands,
                        &mut materials,
                        &mut meshes,
                    );
                }
            }
            CollisionEvent::Border => {
                let snake_entity = snake.single();
                commands.entity(snake_entity).despawn();
                for (_, bonus_entity, _) in bonus.iter() {
                    commands.entity(bonus_entity).despawn();
                }
                for queue_entity in queue.iter() {
                    commands.entity(queue_entity).despawn();
                }
                for border_entity in border_query.iter_mut() {
                    commands.entity(border_entity).despawn();
                }
                extra_bonus_timer.reset();
                game_state.set(GameState::Over).unwrap();
            }
        }
    }
}

fn enter_pause(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::P]) {
        game_state.push(GameState::Paused).unwrap();
        keyboard_input.reset(KeyCode::P);
        keyboard_input.reset(KeyCode::Space);
    }
}

fn resume_game(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::P]) {
        game_state.push(GameState::Running).unwrap();
        keyboard_input.reset(KeyCode::P);
        keyboard_input.reset(KeyCode::Space);
    }
}

fn restart_game(
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

fn compute_borders_visibility(
    game_state: ResMut<State<GameState>>,
    mut border_query: Query<&mut Visibility, With<Border>>,
) {
    let borders_visibility = game_state.current().are_borders_visible();
    for mut visibility in border_query.iter_mut() {
        visibility.is_visible = borders_visibility;
    }
}

fn set_first_direction(
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

fn handle_snake_direction_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Snake>) {
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

    let mut snake = query.single_mut();
    if let (Some(current_snake_direction), Some(new_direction_unwrapped)) =
        (snake.direction, new_direction)
    {
        if !current_snake_direction.conflicts_with(new_direction_unwrapped) {
            snake.direction = new_direction;
        }
    }
}
