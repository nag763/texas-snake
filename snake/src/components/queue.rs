use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::common::*;

use super::prelude::{Collider, Spawnable};

/// The queue grows as long as the snake eats bonuses.
#[derive(Debug, Component, Default, Copy, Clone)]
pub enum Queue {
    /// First queue component, isn't collidable
    First,
    /// Others queue members
    #[default]
    Other
}

impl Spawnable<MaterialMesh2dBundle<ColorMaterial>> for Queue {
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
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            ..default()
        }
    }

    fn additional_systems(&self, commands: &mut bevy::ecs::system::EntityCommands) {
        match self {
            Queue::First => {},
            Queue::Other => {commands.insert(Collider);}
        }
    }
}
