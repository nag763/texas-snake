use bevy::time::FixedTimestep;
use bevy::{prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle};

const PADDLE_SPEED_FACTOR: f32 = 5.;
const PADDLE_DISTANCE_FACTOR: f32 = 0.4;
const BALL_DIAMETER: f32 = 32.;
const SCREEN_HEIGHT: f32 = 720.;
const SCREEN_WIDTH: f32 = 480.;
const TIME_STEP: f64 = 0.01;
const BALL_TRANSLATION_PER_STEP: f32 = 2.5;
const BALL_DEFECTION_FACTOR: f32 = 40.;
const BORDER_SPLAT_SIZE : f32 = 1.0f32;

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
                .with_system(lower_paddle_move_system.before(check_bounds))
                .with_system(higher_paddle_move_system.before(check_bounds))
                .with_system(ball_velocity.before(check_bounds)),
        )
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
        .insert(LowerPaddle)
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
        .insert(HigherPaddle)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: PADDLE_DIMENSIONS,
                    flip: true,
                }))
                .into(),
            transform: Transform::default().with_translation(Vec3 {
                y: HIGHER_PADDLE_Y_AXIS,
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
            angle: 0.,
        })
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(BALL_DIAMETER)),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            ..default()
        });
    let mut brick_maker_closure = |x: f32, y: f32| {
        commands
            .spawn()
            .insert(Border)
            .insert_bundle(SpriteBundle {
                sprite: Sprite::default(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::splat(BORDER_SPLAT_SIZE),
                    ..default()
                },
                ..default()
            })
            .insert(Collider);
    };
    for i in LOWER_PADDLE_Y_AXIS.floor() as i32..HIGHER_PADDLE_Y_AXIS.floor() as i32 {
        brick_maker_closure(SCREEN_WIDTH / 2., i as f32);
        brick_maker_closure(-SCREEN_WIDTH / 2., i as f32);
    }
}

fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(SCREEN_WIDTH, SCREEN_HEIGHT);
}

#[derive(Component)]
struct LowerPaddle;

#[derive(Component)]
struct HigherPaddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Border;

#[derive(Component)]
struct Collider;

#[derive(Component, Default)]
struct BallVelocity {
    direction: f32,
    speed: f32,
    angle: f32,
}

fn check_bounds(
    mut ball: Query<(&Transform, &mut BallVelocity), With<Ball>>,
    collider: Query<(&Transform, Option<&Border>), With<Collider>>,
) {
    let (transform, mut velocity) = ball.single_mut();
    let (ball_x, ball_y) = (transform.translation.x, transform.translation.y);

    for (collider_transform, maybe_border) in collider.iter() {
        let paddle_position = collider_transform.translation;

        let dimensions = match maybe_border {
            Some(_) => Vec2::splat(BORDER_SPLAT_SIZE),
            None => PADDLE_DIMENSIONS
        };

        let collide = collide(
            transform.translation,
            Vec2::splat(BALL_DIAMETER),
            paddle_position,
            dimensions
        );
        if collide.is_some() {
            let (paddle_x, paddle_y) = (paddle_position.x, paddle_position.y);
            if maybe_border.is_none() {
                velocity.direction *= -1.;
                velocity.speed *= 1.1;
                velocity.angle = Vec2::new(paddle_x, paddle_y).angle_between(Vec2::new(ball_x, ball_y)) * BALL_DEFECTION_FACTOR;
            } else {
                velocity.angle*=-1.;
            }   
        }
    }
}

fn ball_velocity(mut ball_query: Query<(&mut Transform, &BallVelocity)>) {
    for (mut transform, velocity) in &mut ball_query {
        transform.translation.x += velocity.angle;
        transform.translation.y += velocity.speed * velocity.direction;
    }
}

fn lower_paddle_move_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<LowerPaddle>>,
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

fn higher_paddle_move_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<HigherPaddle>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut new_x_paddle = paddle_transform.translation.x;
    if keyboard_input.pressed(KeyCode::D) {
        new_x_paddle += 1.0 * PADDLE_SPEED_FACTOR;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        new_x_paddle -= 1.0 * PADDLE_SPEED_FACTOR;
    }
    if f32::abs(new_x_paddle) < (SCREEN_WIDTH - PADDLE_DIMENSIONS.x) / 2. {
        paddle_transform.translation.x = new_x_paddle;
    }
}
