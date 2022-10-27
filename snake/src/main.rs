use rand::Rng;

use bevy::{
    app::AppExit, prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle,
    time::FixedTimestep,
};

/// How many times per seconds the system does an action.
const TIME_STEP: f64 = 0.02;

/// The screen height.
const SCREEN_HEIGHT: f32 = 480.;
const MAX_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.;
const MIN_SCREEN_HEIGHT: f32 = -MAX_SCREEN_HEIGHT;

/// The screen width.
const SCREEN_WIDTH: f32 = 640.;
const MAX_SCREEN_WIDTH: f32 = SCREEN_WIDTH / 2.;
const MIN_SCREEN_WIDTH: f32 = -MAX_SCREEN_WIDTH;

/// The snake head size, same size for each queue member
const SNAKE_SIZE: f32 = 10f32;
/// The snake dimensions
const SNAKE_DIMENSIONS: Vec2 = Vec2::splat(SNAKE_SIZE);
/// The snake speed
const SNAKE_SPEED_FACTOR: f32 = (SNAKE_SIZE + 5f32) * 0.40;

/// The bonus diameter
const BONUS_DIAMETER: f32 = 10f32;

/// The size of each border
const BORDER_SIZE: f32 = 15f32;

/// The snake direction in a 2D plan
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl SnakeDirection {
    /// Get the conflictual direction.
    ///
    /// ie, going upward is impossible for the snake if he is already
    /// going down.
    fn get_conflictual_direction(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// Returns whether the current position conflicts with another.
    fn conflicts_with(&self, other: Self) -> bool {
        self.get_conflictual_direction() == other
    }

    /// Returns the current direction as a translatable vec.
    fn into_translation(self) -> Vec3 {
        match self {
            Self::Up => Vec3::new(0., 1., 0.),
            Self::Down => Vec3::new(0., -1., 0.),
            Self::Right => Vec3::new(1., 0., 0.),
            Self::Left => Vec3::new(-1., 0., 0.),
        }
    }
}

/// The snake is the player.
#[derive(Debug, Component, Default)]
struct Snake {
    /// The size of the snake is the number of times he got a bonus.
    ///
    /// For each bonus => the size increases.
    size: u32,
    /// The snake direction.
    direction: Option<SnakeDirection>,
    /// Its last position.
    last_position: Vec3,
}

/// The snake's queue.
#[derive(Debug, Component)]
struct Queue;

/// The limit of the game.
#[derive(Debug, Default, Component)]
struct Border;

/// A collider is something the snake can't go through.
///
/// It can either be a bonus, or a border.
#[derive(Debug, Default, Component)]
struct Collider;

/// The event following a conflict of position between the snake and a collider.
#[derive(Default)]
enum CollisionEvent {
    #[default]
    Border,
    Bonus(u32),
}

/// A bonus once collided with the snake will increase its size, and thus the
/// player's score.
#[derive(Component, Default)]
struct Bonus {
    /// Whether the bonus is an extra bonus.
    extra_bonus: bool,
}

/// Computes a random translation inside a box defined by two position in each
/// axis.
fn compute_random_translation_inside(x0: f32, x1: f32, y0: f32, y1: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(x0..x1);
    let y = rng.gen_range(y0..y1);
    Vec3 { x, y, ..default() }
}

fn compute_random_bonus_position() -> Vec3 {
    compute_random_translation_inside(
        MIN_SCREEN_WIDTH + BORDER_SIZE,
        MAX_SCREEN_WIDTH - BORDER_SIZE,
        MIN_SCREEN_HEIGHT + BORDER_SIZE,
        MAX_SCREEN_HEIGHT - BORDER_SIZE,
    )
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_event::<CollisionEvent>()
        .add_startup_system(setup)
        .add_system(window_resize_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(collision_handler)
                .with_system(change_snake_direction)
                .with_system(check_collisions)
                .with_system(
                    move_snake
                        .before(change_snake_direction)
                        .after(collision_handler),
                )
                .with_system(move_queue),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let bonus_initial_position = compute_random_bonus_position();
    // The camera
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn()
        .insert(Snake::default())
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: SNAKE_DIMENSIONS,
                    ..default()
                }))
                .into(),
            transform: Transform::default(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        });
    // The first bonus
    commands
        .spawn()
        .insert(Bonus::default())
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default()
                .with_scale(Vec3::splat(BONUS_DIAMETER))
                .with_translation(bonus_initial_position),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(Collider);
    // The snake head
    commands
        .spawn()
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::splat(1f32),
                    flip: false,
                }))
                .into(),
            transform: Transform::default()
                .with_translation(Vec3 {
                    x: MIN_SCREEN_WIDTH,
                    ..default()
                })
                .with_scale(Vec3::new(BORDER_SIZE, SCREEN_HEIGHT, 0f32)),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(Collider);
    // The borders
    commands
        .spawn()
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::splat(1f32),
                    flip: true,
                }))
                .into(),
            transform: Transform::default()
                .with_translation(Vec3 {
                    x: MAX_SCREEN_WIDTH,
                    ..default()
                })
                .with_scale(Vec3::new(BORDER_SIZE, SCREEN_HEIGHT, 0f32)),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(Collider);
    commands
        .spawn()
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::splat(1f32),
                    flip: false,
                }))
                .into(),
            transform: Transform::default()
                .with_translation(Vec3 {
                    y: MAX_SCREEN_HEIGHT,
                    ..default()
                })
                .with_scale(Vec3::new(SCREEN_WIDTH, BORDER_SIZE, 0f32)),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(Collider);
    commands
        .spawn()
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::splat(1f32),
                    flip: true,
                }))
                .into(),
            transform: Transform::default()
                .with_translation(Vec3 {
                    y: MIN_SCREEN_HEIGHT,
                    ..default()
                })
                .with_scale(Vec3::new(SCREEN_WIDTH, BORDER_SIZE, 0f32)),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(Collider);
}

/// Resizes the window at startup.
fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(SCREEN_WIDTH, SCREEN_HEIGHT);
}

/// The movement of snake per TIME_STEP applied to the ball.
fn move_snake(mut query: Query<(&mut Transform, &mut Snake)>) {
    let (mut transform, mut snake) = query.single_mut();
    if let Some(direction) = snake.direction {
        let translation_diff = direction.into_translation() * SNAKE_SPEED_FACTOR;
        snake.last_position = transform.translation;
        transform.translation += translation_diff;
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
                let points = match bonus.extra_bonus {
                    true => 5,
                    false => 1,
                };
                collision_event_writer.send(CollisionEvent::Bonus(points));
            } else {
                collision_event_writer.send_default();
            }
        }
    }
}

fn collision_handler(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut exit: EventWriter<AppExit>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut bonus: Query<&mut Transform, With<Bonus>>,
    mut snake: Query<&mut Snake>,
) {
    if !collision_event_reader.is_empty() {
        let mut snake = snake.single_mut();
        for event in collision_event_reader.iter() {
            match event {
                CollisionEvent::Bonus(points) => {
                    // spawn a queue
                    commands
                        .spawn()
                        .insert(Queue)
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(Mesh::from(shape::Quad {
                                    size: SNAKE_DIMENSIONS,
                                    ..default()
                                }))
                                .into(),
                            transform: Transform::default().with_translation(Vec3::new(
                                MIN_SCREEN_WIDTH,
                                MIN_SCREEN_HEIGHT,
                                0f32,
                            )),
                            material: materials.add(ColorMaterial::from(Color::GRAY)),
                            ..default()
                        })
                        .insert(Collider);
                    snake.size += points;
                    let mut bonus_position = bonus.single_mut();
                    bonus_position.translation = compute_random_bonus_position();
                }
                CollisionEvent::Border => {
                    exit.send(AppExit);
                    println!("Score {}", snake.size);
                    break;
                }
            }
        }
        collision_event_reader.clear();
    }
}

fn change_snake_direction(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Snake>) {
    let mut snake = query.single_mut();

    let mut new_direction: Option<SnakeDirection> = None;

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        new_direction = Some(SnakeDirection::Right);
    }
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Q) {
        new_direction = Some(SnakeDirection::Left);
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Z) {
        new_direction = Some(SnakeDirection::Up);
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        new_direction = Some(SnakeDirection::Down);
    }

    // If no current direction => take the value of any new direction
    if snake.direction.is_none() {
        snake.direction = new_direction;
    // If a current direction => ensure there is already a no direction (the
    // snake can't stop itself)
    } else if let (Some(current_snake_direction), Some(new_direction_unwrapped)) =
        (snake.direction, new_direction)
    {
        if !current_snake_direction.conflicts_with(new_direction_unwrapped) {
            snake.direction = new_direction;
        }
    }
}
