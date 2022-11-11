use crate::common::*;
use crate::components::prelude::*;
use bevy::{prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle};

use rand::Rng;
use std::fmt;

/// A border set is a preset of borders that will be spawned during the game.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Component)]
pub enum BorderSet {
    /// Borders are the screen limit.
    Screen,
    /// Borders are a cross that cross each others
    /// in the middle of the screen.
    Cross,
    /// One single border that is the width of the screen and
    /// located in the middle of the screeen.
    Horizontal,
    /// Same as horizontal, but with the height of the screen.
    Vertical,
}

impl fmt::Display for BorderSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl BorderSet {
    /// Returns all the possible border sets.
    pub fn iterator() -> impl Iterator<Item = Self> {
        [
            BorderSet::Screen,
            BorderSet::Cross,
            BorderSet::Horizontal,
            BorderSet::Vertical,
        ]
        .into_iter()
    }

    /// Defines the possible borders for each border set.
    pub fn get_borders(&self) -> Vec<Transform> {
        match self {
            BorderSet::Screen => vec![
                Transform::default()
                    .with_translation(Vec3 {
                        y: MAX_SCREEN_HEIGHT,
                        ..default()
                    })
                    .with_scale(Vec3::new(SCREEN_WIDTH, BORDER_SIZE, 0f32)),
                Transform::default()
                    .with_translation(Vec3 {
                        y: MIN_SCREEN_HEIGHT,
                        ..default()
                    })
                    .with_scale(Vec3::new(SCREEN_WIDTH, BORDER_SIZE, 0f32)),
                Transform::default()
                    .with_translation(Vec3 {
                        x: MAX_SCREEN_WIDTH,
                        ..default()
                    })
                    .with_scale(Vec3::new(BORDER_SIZE, SCREEN_HEIGHT, 0f32)),
                Transform::default()
                    .with_translation(Vec3 {
                        x: MIN_SCREEN_WIDTH,
                        ..default()
                    })
                    .with_scale(Vec3::new(BORDER_SIZE, SCREEN_HEIGHT, 0f32)),
            ],
            BorderSet::Horizontal => {
                vec![Transform::default().with_scale(Vec3::new(BORDER_SIZE, SCREEN_HEIGHT, 0f32))]
            }
            BorderSet::Vertical => {
                vec![Transform::default().with_scale(Vec3::new(SCREEN_WIDTH, BORDER_SIZE, 0f32))]
            }
            BorderSet::Cross => vec![
                Transform::default().with_scale(Vec3::new(BORDER_SIZE, SCREEN_HEIGHT, 0f32)),
                Transform::default().with_scale(Vec3::new(SCREEN_WIDTH, BORDER_SIZE, 0f32)),
            ],
        }
    }

    /// Compute the random non collidable position for the given border set.
    ///
    /// This is useful when you need to spawn a bonus randomly for instance.
    pub fn compute_random_bonus_position(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let bonus_dimensions = Vec2::splat(BONUS_DIAMETER);
        // We loop until a bonus position is returned
        'generator: loop {
            // It has to be spawned within the screen limits ...
            let x = rng.gen_range(MIN_SCREEN_WIDTH..MAX_SCREEN_WIDTH);
            let y = rng.gen_range(MIN_SCREEN_HEIGHT..MAX_SCREEN_HEIGHT);
            let random_position = Vec3::new(x, y, 0f32);
            // ... and checked that it doesn't collide with any of the borders.
            for border in self.get_borders() {
                let border_dimensions = Vec2::new(border.scale.x, border.scale.y);
                if collide(
                    border.translation,
                    border_dimensions,
                    random_position,
                    bonus_dimensions,
                )
                .is_some()
                {
                    continue 'generator;
                }
            }
            return random_position;
        }
    }

    /// The snake initial position depends of the border set,
    /// this methods returns a good snake position for each of
    /// the border set.
    pub fn get_snake_initial_position(&self) -> Vec3 {
        match self {
            BorderSet::Screen => Vec3::default(),
            _ => Vec3 {
                x: -150f32,
                y: 150f32,
                ..default()
            },
        }
    }

    /// Spawn the borders in the given app.
    pub fn spawn_borders(
        &self,
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        for border in self.get_borders() {
            commands
                .spawn()
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad {
                            size: Vec2::splat(1f32),
                            flip: false,
                        }))
                        .into(),
                    transform: border,
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    ..default()
                })
                .insert(Collider)
                .insert(Border);
        }
    }
}
