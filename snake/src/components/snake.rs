use super::spawnable::Spawnable;
use crate::common::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

/// The snake direction in a 2D plan
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl SnakeDirection {
    /// Get the conflictual direction.
    ///
    /// ie, going upward is impossible for the snake if he is already
    /// going down.
    fn get_conflictual_direction(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// Returns whether the current position conflicts with another.
    pub fn conflicts_with(&self, other: Self) -> bool {
        self.get_conflictual_direction() == other
    }

    /// Returns the current direction as a translatable vec.
    pub fn into_translation(self) -> Vec3 {
        match self {
            Self::Up => Vec3::new(0., 1., 0.),
            Self::Down => Vec3::new(0., -1., 0.),
            Self::Right => Vec3::new(1., 0., 0.),
            Self::Left => Vec3::new(-1., 0., 0.),
        }
    }
}

/// The snake is the player.
#[derive(Debug, Component, Default, Copy, Clone)]
pub struct Snake {
    /// The snake direction.
    pub direction: Option<SnakeDirection>,
    /// Its last position.
    pub last_position: Vec3,
}

impl Spawnable<MaterialMesh2dBundle<ColorMaterial>> for Snake {
    fn get_bundle(
        &self,
        transform: Transform,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: SNAKE_DIMENSIONS,
                    ..default()
                }))
                .into(),
            transform,
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        }
    }
    fn additional_systems(_commands: &mut bevy::ecs::system::EntityCommands) {}
}
