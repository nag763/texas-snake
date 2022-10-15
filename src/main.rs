use bevy::time::FixedTimestep;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const PADDLE_SPEED_FACTOR: f32 = 5.;
const BALL_DIAMETER: f32 = 32.;
const SCREEN_HEIGHT: f32 = 720.;
const SCREEN_WIDTH: f32 = 480.;
const TIME_STEP: f64 = 0.01;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(window_resize_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(ball_fall)
                .with_system(ball_velocity),
        )
        .add_system(paddle_move_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn()
        .insert(Paddle)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2 { x: 1., y: 0.25 },
                    flip: true,
                }))
                .into(),
            transform: Transform::default().with_scale(Vec3::splat(128.)),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        });
    commands
        .spawn()
        .insert(Ball)
        .insert(BallVelocity {
            direction: -1.,
            speed: 0.,
        })
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(BALL_DIAMETER)),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            ..default()
        });
}

fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(SCREEN_WIDTH, SCREEN_HEIGHT);
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Default)]
struct BallVelocity {
    direction: f32,
    speed: f32,
}

fn ball_fall(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut BallVelocity), With<Ball>>,
) {
    for (transform, mut velocity) in &mut ball_query {
        let y_position = transform.translation.y;
        if f32::abs(y_position) < (SCREEN_HEIGHT - BALL_DIAMETER) / 2. {
            velocity.speed = time.seconds_since_startup() as f32;
        } else {
            velocity.direction *= -1.;
        }
    }
}

fn ball_velocity(mut ball_query: Query<(&mut Transform, &BallVelocity)>) {
    for (mut transform, velocity) in &mut ball_query {
        transform.translation.y += velocity.speed * velocity.direction;
    }
}

fn paddle_move_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();
    if keyboard_input.pressed(KeyCode::Right) {
        paddle_transform.translation.x += 1.0 * PADDLE_SPEED_FACTOR;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        paddle_transform.translation.x -= 1.0 * PADDLE_SPEED_FACTOR;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        paddle_transform.translation.y += 1.0 * PADDLE_SPEED_FACTOR;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        paddle_transform.translation.y -= 1.0 * PADDLE_SPEED_FACTOR;
    }
}
