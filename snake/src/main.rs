use rand::Rng;

use bevy::{
    app::AppExit, prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle,
    time::FixedTimestep,
};

/// How many times per seconds the system does an action.
const TIME_STEP: f64 = 0.01;

/// The screen height.
const SCREEN_HEIGHT: f32 = 480.;
const MAX_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.;
const MIN_SCREEN_HEIGHT: f32 = -MAX_SCREEN_HEIGHT;

/// The screen width.
const SCREEN_WIDTH: f32 = 640.;
const MAX_SCREEN_WIDTH: f32 = SCREEN_WIDTH / 2.;
const MIN_SCREEN_WIDTH: f32 = -MAX_SCREEN_WIDTH;

const SNAKE_SIZE: f32 = 10f32;
const SNAKE_DIMENSIONS: Vec2 = Vec2::splat(SNAKE_SIZE);

const BORDER_SIZE: f32 = 10f32;
const BORDER_DIMENSIONS: Vec2 = Vec2::splat(BORDER_SIZE);

const BONUS_DIAMETER: f32 = 10.;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl SnakeDirection {
    fn get_conflictual_direction(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn conflicts_with(&self, other: Self) -> bool {
        self.get_conflictual_direction() == other
    }

    fn into_translation(self) -> Vec3 {
        match self {
            Self::Up => Vec3::new(0., 1., 0.),
            Self::Down => Vec3::new(0., -1., 0.),
            Self::Right => Vec3::new(1., 0., 0.),
            Self::Left => Vec3::new(-1., 0., 0.),
        }
    }
}

#[derive(Debug, Component, Default)]
struct Snake {
    size: u32,
    direction: Option<SnakeDirection>,
}

#[derive(Debug, Default, Component)]
struct Border;

#[derive(Debug, Default, Component)]
struct Collider;

#[derive(Default)]
enum CollisionEvent {
    #[default]
    Border,
    Bonus(u32),
}

#[derive(Component, Default)]
struct Bonus {
    extra_bonus: bool,
}

fn compute_random_translation_inside(x0: f32, x1: f32, y0: f32, y1: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(x0..x1);
    let y = rng.gen_range(y0..y1);
    Vec3 { x, y, ..default() }
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
                .with_system(change_snake_direction)
                .with_system(check_collisions)
                .with_system(collision_handler)
                .with_system(move_snake.before(change_snake_direction)),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let bonus_initial_position = compute_random_translation_inside(
        MIN_SCREEN_WIDTH + BORDER_SIZE,
        MAX_SCREEN_WIDTH - BORDER_SIZE,
        MIN_SCREEN_HEIGHT + BORDER_SIZE,
        MAX_SCREEN_HEIGHT - BORDER_SIZE,
    );
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

    let mut border_maker_closure = |x: f32, y: f32| {
        commands
            .spawn()
            .insert(Border::default())
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad {
                        size: BORDER_DIMENSIONS,
                        ..default()
                    }))
                    .into(),
                transform: Transform::default().with_translation(Vec3::new(x, y, 0f32)),
                material: materials.add(ColorMaterial::from(Color::GRAY)),
                ..default()
            })
            .insert(Collider);
    };

    for x in ((MIN_SCREEN_WIDTH.floor() as isize)..(MAX_SCREEN_WIDTH.floor() as isize))
        .step_by(BORDER_SIZE as usize)
    {
        border_maker_closure(x as f32, MIN_SCREEN_HEIGHT);
        border_maker_closure(x as f32, MAX_SCREEN_HEIGHT);
    }

    for y in ((MIN_SCREEN_HEIGHT.floor() as isize)..(MAX_SCREEN_HEIGHT.floor() as isize))
        .step_by(BORDER_SIZE as usize)
    {
        border_maker_closure(MIN_SCREEN_WIDTH, y as f32);
        border_maker_closure(MAX_SCREEN_WIDTH, y as f32);
    }
}

/// Resizes the window at startup.
fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(SCREEN_WIDTH, SCREEN_HEIGHT);
}

/// The movement of snake per TIME_STEP applied to the ball.
fn move_snake(mut query: Query<(&mut Transform, &Snake)>) {
    let (mut transform, snake) = query.single_mut();
    if let Some(direction) = snake.direction {
        let translation_diff = direction.into_translation();
        transform.translation += translation_diff;
    }
}

fn check_collisions(
    snake: Query<&Transform, With<Snake>>,
    colliders: Query<(&Transform, Option<&Bonus>), With<Collider>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    let snake_position = snake.single().translation;
    for (collider, maybe_bonus) in colliders.iter() {
        let collide = collide(
            snake_position,
            SNAKE_DIMENSIONS,
            collider.translation,
            BORDER_DIMENSIONS,
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
    mut exit: EventWriter<AppExit>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut bonus: Query<&mut Transform, With<Bonus>>,
    mut snake: Query<&mut Snake>,
) {
    if !collision_event_reader.is_empty() {
        for event in collision_event_reader.iter() {
            match event {
                CollisionEvent::Bonus(points) => {
                    let mut bonus_position = bonus.single_mut();
                    bonus_position.translation = compute_random_translation_inside(
                        MIN_SCREEN_WIDTH + BORDER_SIZE,
                        MAX_SCREEN_WIDTH - BORDER_SIZE,
                        MIN_SCREEN_HEIGHT + BORDER_SIZE,
                        MAX_SCREEN_HEIGHT - BORDER_SIZE,
                    );
                    let mut snake = snake.single_mut();
                    snake.size += points;
                }
                CollisionEvent::Border => {
                    exit.send(AppExit);
                }
            }
        }
        collision_event_reader.clear();
    }
}

fn change_snake_direction(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Snake>) {
    let mut snake = query.single_mut();

    let mut new_direction: Option<SnakeDirection> = None;

    if keyboard_input.pressed(KeyCode::Right) {
        new_direction = Some(SnakeDirection::Right);
    }
    if keyboard_input.pressed(KeyCode::Left) {
        new_direction = Some(SnakeDirection::Left);
    }
    if keyboard_input.pressed(KeyCode::Up) {
        new_direction = Some(SnakeDirection::Up);
    }
    if keyboard_input.pressed(KeyCode::Down) {
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
