use crate::common::*;
use crate::components::prelude::*;
use crate::init_game_components;
use bevy::{prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle};

use rand::Rng;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Component)]
pub enum BorderSet {
    Screen,
    Cross,
    Horizontal,
    Vertical,
}

impl fmt::Display for BorderSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl BorderSet {
    pub fn iterator() -> impl Iterator<Item = Self> {
        [
            BorderSet::Screen,
            BorderSet::Cross,
            BorderSet::Horizontal,
            BorderSet::Vertical,
        ]
        .into_iter()
    }

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

    pub fn compute_random_bonus_position(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let bonus_dimensions = Vec2::splat(BONUS_DIAMETER);
        'generator: loop {
            let x = rng.gen_range(MIN_SCREEN_WIDTH..MAX_SCREEN_WIDTH);
            let y = rng.gen_range(MIN_SCREEN_HEIGHT..MAX_SCREEN_HEIGHT);
            let random_position = Vec3::new(x, y, 0f32);
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
        init_game_components(commands, materials, meshes, *self);
    }
}
