mod common;
mod components;
mod resources;
mod systems;

use common::*;
use components::prelude::*;
use resources::prelude::*;
use systems::prelude::*;

use rand::Rng;

use bevy::{app::AppExit, prelude::*, time::Stopwatch};

#[derive(Debug, Component)]
struct UserText;

/// The event following a conflict of position between the snake and a collider.
#[derive(Default)]
pub enum CollisionEvent {
    #[default]
    Border,
    Bonus(u32),
}

/// Timer for extra bonuses.
#[derive(Clone, Deref, DerefMut, Default)]
pub struct ExtraBonusTimer(Stopwatch);

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

/// Loads the assets at startup.
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

/// Resizes the window at startup.
fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(APP_TITLE.into());
    window.set_resolution(SCREEN_WIDTH, SCREEN_HEIGHT);
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

/// Every collision event handling.
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
            // If a bonus is collided, we increase the length of the queue, and
            // others subsequent actions.
            CollisionEvent::Bonus(points) => {
                for _ in 0..*points {
                    // The queue is spawned out of the screen, and then moved
                    // by the systems.
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
                **score += points;
                let mut extra_bonus_exists: bool = false;
                // For each bonus, given the number of points, we proceed to ...
                for (mut bonus_position, bonus_entity, bonus) in bonus.iter_mut() {
                    match bonus {
                        // Change its position if it a normal one that has been touched.
                        Bonus::Normal if points == &1u32 => {
                            bonus_position.translation =
                                border_set.unwrap().compute_random_bonus_position();
                        }
                        // Despawn it if it is an extra bonus.
                        Bonus::ExtraBonus if points == &5u32 => {
                            extra_bonus_timer.reset();
                            commands.entity(bonus_entity).despawn();
                        }
                        // If the normal bonus is touched while the extra bonus is touched, we don't
                        // do anything.
                        Bonus::ExtraBonus if points == &1u32 => {
                            extra_bonus_exists = true;
                        }
                        _ => (),
                    }
                }
                let mut rng = rand::thread_rng();
                // If no extra bonus has been touched and none are on screen atm, we roll the dice
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
            // If a border is collided, we despawn all the game components
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
