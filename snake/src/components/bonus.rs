use super::{prelude::Collider, spawnable::Spawnable};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

/// A bonus once collided with the snake will increase its size, and thus the
/// player's score.
#[derive(Component, Default, Clone, Copy, Debug)]
pub enum Bonus {
    /// A normal bonus only increase the player score by a single point.
    ///
    /// Once a normal bonus is collided, another one spawns indefinitely.
    #[default]
    Normal,
    /// An extra bonus is a rare bonus that will increase the player by more points.
    ///
    /// Extra bonuses are rare and appear randomly once a normal bonus is collided by the snake.
    ExtraBonus,
}

impl Bonus {
    /// Returns the number of points that are rewarded for
    /// colliding with the bonus.
    pub fn get_points(&self) -> u32 {
        match self {
            Bonus::Normal => 1,
            Bonus::ExtraBonus => 5,
        }
    }
}

impl Spawnable<MaterialMesh2dBundle<ColorMaterial>> for Bonus {
    fn get_bundle(
        &self,
        transform: Transform,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        let bonus_color: Color = match self {
            Bonus::Normal => Color::WHITE,
            Bonus::ExtraBonus => Color::GOLD,
        };
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform,
            material: materials.add(ColorMaterial::from(bonus_color)),
            ..default()
        }
    }

    fn additional_systems(&self, commands: &mut bevy::ecs::system::EntityCommands) {
        commands.insert(Collider);
    }
}
