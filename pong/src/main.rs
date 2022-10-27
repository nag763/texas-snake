use bevy::time::FixedTimestep;
use bevy::{prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle};

/// How many times per seconds the system does an action.
const TIME_STEP: f64 = 0.01;
/// The screen height.
const SCREEN_HEIGHT: f32 = 720.;
/// The screen width.
const SCREEN_WIDTH: f32 = 480.;
/// The score's font size.
const FONT_SIZE: f32 = 15f32;
/// The score's padding.
const FONT_PADDING: Val = Val::Px(5.);

/// The paddle speed factor, or how much one key stroke moves the paddle.
const PADDLE_SPEED_FACTOR: f32 = 5.;
/// The distance separating the paddle from the middle of the screen.
const PADDLE_DISTANCE_FACTOR: f32 = 0.4;
/// The paddle's dimensions.
const PADDLE_DIMENSIONS: Vec2 = Vec2::new(128., 0.175 * 128.);
/// The higher paddle y axis.
const HIGHER_PADDLE_Y_AXIS: f32 = SCREEN_HEIGHT * PADDLE_DISTANCE_FACTOR;
/// The lower paddle y axis.
const LOWER_PADDLE_Y_AXIS: f32 = -HIGHER_PADDLE_Y_AXIS;

/// The ball diameter.
const BALL_DIAMETER: f32 = 32.;
/// The ball translation per time step.
const BALL_TRANSLATION_PER_STEP: f32 = 2.5;
/// How much a hit between the paddle and the ball will deflect.
const BALL_DEFLECTION_FACTOR: f32 = 40.;
/// Each border size.
const BORDER_SPLAT_SIZE: f32 = 1.0f32;
/// How much the ball is fasten everytime it touches a paddle.
const SPEED_INCREASE_ON_TOUCH: f32 = 1.1;

fn main() {
    // We first create the two paddle systems from the create_paddle_move_system Fn
    let low_paddle_system = create_paddle_move_system::<LowerPaddle>(KeyCode::Left, KeyCode::Right);
    let upper_paddle_system = create_paddle_move_system::<HigherPaddle>(KeyCode::Q, KeyCode::D);

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(ScoreSheet::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(window_resize_system)
        .add_event::<OutOfBoundsEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(check_bounds)
                .with_system(low_paddle_system.before(check_bounds))
                .with_system(upper_paddle_system.before(check_bounds))
                .with_system(ball_velocity.before(check_bounds))
                .with_system(out_of_bounds_event.before(check_bounds)),
        )
        .add_system(update_scoresheet)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // The camera
    commands.spawn_bundle(Camera2dBundle::default());
    // The blue player's score
    commands.spawn_bundle(
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: FONT_SIZE,
            color: Color::BLUE,
            font: asset_server.load("score_font.otf"),
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: FONT_PADDING,
                left: FONT_PADDING,
                ..default()
            },
            ..default()
        }),
    );
    // The red player's score
    commands.spawn_bundle(
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: FONT_SIZE,
            color: Color::RED,
            font: asset_server.load("score_font.otf"),
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: FONT_PADDING,
                right: FONT_PADDING,
                ..default()
            },
            ..default()
        }),
    );
    // The red player's paddle
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
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..default()
        })
        .insert(Collider);
    // The blue player's paddle
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
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..default()
        })
        .insert(Collider);
    // The ball
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

    // The closure to create bricks for a given x and y position
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
                visibility: Visibility { is_visible: false },
                ..default()
            })
            .insert(Collider);
    };
    // And then we create a border everywhere in the right and left between the
    // upper and lower paddle.
    for i in LOWER_PADDLE_Y_AXIS.floor() as i32..HIGHER_PADDLE_Y_AXIS.floor() as i32 {
        brick_maker_closure(SCREEN_WIDTH / 2., i as f32);
        brick_maker_closure(-SCREEN_WIDTH / 2., i as f32);
    }
}

/// Resizes the window at startup.
fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(SCREEN_WIDTH, SCREEN_HEIGHT);
}

/// The lower paddle.
#[derive(Component)]
struct LowerPaddle;

/// The higher paddle.
#[derive(Component)]
struct HigherPaddle;

/// The ball.
#[derive(Component)]
struct Ball;

/// One border.
#[derive(Component)]
struct Border;

/// The collider system, used by everything that will interact with the ball.
#[derive(Component)]
struct Collider;

/// The oob event, used to inform other systems that the balls is out of the
/// limits of the game.
#[derive(Default, Component)]
struct OutOfBoundsEvent;

/// The score sheet.
#[derive(Default, Debug)]
struct ScoreSheet {
    /// Red player's score.
    red: u32,
    /// Blue player's score.
    blue: u32,
}

/// The ball's velocity, used to describe how the ball should move.
#[derive(Component, Default)]
struct BallVelocity {
    /// Direction of the ball.
    ///
    /// Should either be strictly equals to 1 or -1.
    direction: f32,
    /// The speed of the ball.
    ///
    /// How much distance the ball will travel vertically per TIME_STEP.
    speed: f32,
    /// The angle of the ball.
    ///
    /// How much the ball will travel horizontally per TIME_STEP.
    angle: f32,
}

/// Check how the ball is located in the space, and if it is actually colliding
/// with anything.
fn check_bounds(
    mut ball: Query<(&Transform, &mut BallVelocity), With<Ball>>,
    collider: Query<(&Transform, Option<&Border>), With<Collider>>,
    mut oob_event_writer: EventWriter<OutOfBoundsEvent>,
) {
    let (transform, mut velocity) = ball.single_mut();
    let (ball_x, ball_y) = (transform.translation.x, transform.translation.y);

    for (collider_transform, maybe_border) in collider.iter() {
        let collider_position = collider_transform.translation;

        // The dimensions of the border differ from the dimension of the paddle.
        let dimensions = match maybe_border {
            Some(_) => Vec2::splat(BORDER_SPLAT_SIZE),
            None => PADDLE_DIMENSIONS,
        };

        let collide = collide(
            transform.translation,
            Vec2::splat(BALL_DIAMETER),
            collider_position,
            dimensions,
        );

        if collide.is_some() {
            let (collider_x, collider_y) = (collider_position.x, collider_position.y);
            if maybe_border.is_none() {
                velocity.direction *= -1.;
                velocity.speed *= SPEED_INCREASE_ON_TOUCH;
                velocity.angle = Vec2::new(collider_x, collider_y)
                    .angle_between(Vec2::new(ball_x, ball_y))
                    * BALL_DEFLECTION_FACTOR;
            } else {
                velocity.angle *= -1.;
            }
        }
    }
    // Someone scored if the ball is over one of the paddle height level !
    if HIGHER_PADDLE_Y_AXIS < f32::abs(ball_y) {
        oob_event_writer.send_default();
    }
}

/// The movement of ball per TIME_STEP applied to the ball.
fn ball_velocity(mut ball_query: Query<(&mut Transform, &BallVelocity)>) {
    for (mut transform, velocity) in &mut ball_query {
        transform.translation.x += velocity.angle;
        transform.translation.y += velocity.speed * velocity.direction;
    }
}

/// Creates the paddle move system from the given inputs for the given paddle.
fn create_paddle_move_system<T: Component>(
    left: KeyCode,
    right: KeyCode,
) -> impl Fn(Res<Input<KeyCode>>, Query<&mut Transform, With<T>>) {
    move |keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<T>>| {
        let mut paddle_transform = query.single_mut();
        let mut new_x_paddle = paddle_transform.translation.x;

        if keyboard_input.pressed(right) {
            new_x_paddle += 1.0 * PADDLE_SPEED_FACTOR;
        }
        if keyboard_input.pressed(left) {
            new_x_paddle -= 1.0 * PADDLE_SPEED_FACTOR;
        }
        // It's always better to ensure our paddle hasn't reached the limits
        // of the game.
        if f32::abs(new_x_paddle) < (SCREEN_WIDTH - PADDLE_DIMENSIONS.x) / 2. {
            paddle_transform.translation.x = new_x_paddle;
        }
    }
}

/// Updates the displayed scoresheet on the screen.
fn update_scoresheet(scoresheet: Res<ScoreSheet>, mut query: Query<&mut Text>) {
    for (i, mut text) in query.iter_mut().enumerate() {
        let text_val = match i {
            0 => scoresheet.blue,
            1 => scoresheet.red,
            _ => continue,
        }
        .to_string();
        text.sections[0].value = text_val;
    }
}

/// Every action that is ran once the ball is out of bounds.
fn out_of_bounds_event(
    oob_events: EventReader<OutOfBoundsEvent>,
    mut ball_query: Query<(&mut Transform, &mut BallVelocity), With<Ball>>,
    mut scoresheet: ResMut<ScoreSheet>,
) {
    if !oob_events.is_empty() {
        let (mut transform, mut velocity) = ball_query.single_mut();

        // We update the score given where the ball is
        if transform.translation.y < 0f32 {
            scoresheet.blue += 1;
        } else {
            scoresheet.red += 1;
        }

        // We reinit the position of the ball
        transform.translation = Vec3::default();
        // And we reinit both its speed factor and deflection angle
        velocity.angle = 0.;
        velocity.speed = BALL_TRANSLATION_PER_STEP;

        oob_events.clear();
    }
}
