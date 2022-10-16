use bevy::time::FixedTimestep;
use bevy::{prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle};

const PADDLE_SPEED_FACTOR: f32 = 5.;
const PADDLE_DISTANCE_FACTOR: f32 = 0.4;
const BALL_DIAMETER: f32 = 32.;
const SCREEN_HEIGHT: f32 = 720.;
const SCREEN_WIDTH: f32 = 480.;
const TIME_STEP: f64 = 0.0001;
const BALL_TRANSLATION_PER_STEP: f32 = 0.025;

const PADDLE_DIMENSIONS: Vec2 = Vec2 {
    x: 128.,
    y: 0.175 * 128.,
};
const HIGHER_PADDLE_Y_AXIS: f32 = SCREEN_HEIGHT * PADDLE_DISTANCE_FACTOR;
const LOWER_PADDLE_Y_AXIS: f32 = -HIGHER_PADDLE_Y_AXIS;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(window_resize_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(check_bounds)
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
        .insert(Collider)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: PADDLE_DIMENSIONS,
                    flip: true,
                }))
                .into(),
            transform: Transform::default().with_translation(Vec3 {
                y: LOWER_PADDLE_Y_AXIS,
                ..Vec3::default()
            }),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        })
        .insert(Collider);
    commands
        .spawn()
        .insert(Ball)
        .insert(BallVelocity {
            direction: -1.,
            speed: BALL_TRANSLATION_PER_STEP,
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

#[derive(Component)]
struct Collider;

#[derive(Component, Default)]
struct BallVelocity {
    direction: f32,
    speed: f32,
}

fn check_bounds(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(&mut Transform, Entity), With<Ball>>,
        Query<&Transform, With<Collider>>,
    )>,
) {
    let paddle = set.p1();
    let paddle_transform = paddle.single();
    let paddle_position = paddle_transform.translation;

    let mut ball = set.p0();
    let (mut transform, entity) = ball.single_mut();
    let (x_position, y_position) = (transform.translation.x, transform.translation.y);
    let collide = collide(
        transform.translation,
        Vec2::splat(BALL_DIAMETER),
        paddle_position,
        PADDLE_DIMENSIONS,
    );
    if collide.is_some() {
        transform.translation.y = HIGHER_PADDLE_Y_AXIS;
    } else if HIGHER_PADDLE_Y_AXIS < f32::abs(y_position) {
        commands.entity(entity).despawn();
    }
    if SCREEN_WIDTH < f32::abs(x_position) / 2. {
        todo!();
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
    let mut new_x_paddle = paddle_transform.translation.x;
    if keyboard_input.pressed(KeyCode::Right) {
        new_x_paddle += 1.0 * PADDLE_SPEED_FACTOR;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        new_x_paddle -= 1.0 * PADDLE_SPEED_FACTOR;
    }
    if f32::abs(new_x_paddle) < (SCREEN_WIDTH - PADDLE_DIMENSIONS.x) / 2. {
        paddle_transform.translation.x = new_x_paddle;
    }
}
