use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::FixedTimestep};

/// How many times per seconds the system does an action.
const TIME_STEP: f64 = 0.01;
/// The screen height.
const SCREEN_HEIGHT: f32 = 480.;
/// The screen width.
const SCREEN_WIDTH: f32 = 640.;

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
        &self.get_conflictual_direction() == &other
    }

    fn into_translation(&self) -> Vec3 {
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

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(window_resize_system)
        .add_system(change_snake_direction)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(move_snake)
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // The camera
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn()
        .insert(Snake::default())
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(10., 10.),
                    ..default()
                }))
                .into(),
            transform: Transform::default(),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            ..default()
        });
}

/// Resizes the window at startup.
fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(SCREEN_WIDTH, SCREEN_HEIGHT);
}

/// The movement of ball per TIME_STEP applied to the ball.
fn move_snake(mut query: Query<(&mut Transform, &Snake)>) {
    let (mut transform, snake) = query.single_mut();
    if let Some(direction) = snake.direction {
        let new_translation = direction.into_translation();
        transform.translation += new_translation;

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
