use bevy::{
    prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle, time::FixedTimestep, app::AppExit,
};

/// How many times per seconds the system does an action.
const TIME_STEP: f64 = 0.01;
/// The screen height.
const SCREEN_HEIGHT: f32 = 480.;
/// The screen width.
const SCREEN_WIDTH: f32 = 640.;

const SNAKE_SIZE: Vec2 = Vec2::splat(10.);

const BORDER_SIZE: Vec2 = Vec2::splat(1.);

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

#[derive(Debug, Default, Component)]
struct Border;

#[derive(Debug, Default, Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

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
                .with_system(move_snake.before(change_snake_direction)),
        )
        .add_system(game_over)
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
                    size: SNAKE_SIZE,
                    ..default()
                }))
                .into(),
            transform: Transform::default(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        });

    let mut border_maker_closure = |x: f32, y: f32| {
        commands
            .spawn()
            .insert(Border::default())
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad {
                        size: BORDER_SIZE,
                        ..default()
                    }))
                    .into(),
                transform: Transform::default().with_translation(Vec3::new(x, y, 0f32)),
                material: materials.add(ColorMaterial::from(Color::GRAY)),
                ..default()
            })
            .insert(Collider);
    };

    let (max_screen_height, max_screen_width): (i32, i32) = (
        (SCREEN_HEIGHT / 2f32).floor() as i32,
        (SCREEN_WIDTH / 2f32).floor() as i32,
    );
    let (min_screen_height, min_screen_width): (i32, i32) = (-max_screen_height, -max_screen_width);

    for x in min_screen_width..max_screen_width {
        border_maker_closure(x as f32, min_screen_height as f32);
        border_maker_closure(x as f32, max_screen_height as f32);
    }

    for y in min_screen_height..max_screen_height {
        border_maker_closure(min_screen_width as f32, y as f32);
        border_maker_closure(max_screen_width as f32, y as f32);
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
    colliders: Query<&Transform, With<Collider>>,
    mut collision_event_writer: EventWriter<CollisionEvent>
) {
    let snake_position = snake.single().translation;
    for collider in colliders.iter() {
        let collide = collide(
            snake_position,
            SNAKE_SIZE,
            collider.translation,
            BORDER_SIZE,
        );
        if collide.is_some() {
            collision_event_writer.send_default();
        }
    }
}

fn game_over(
    mut exit: EventWriter<AppExit>,
    collision_event_reader: EventReader<CollisionEvent>
) {
    if !collision_event_reader.is_empty() {
        exit.send(AppExit);
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
