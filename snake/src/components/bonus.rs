use super::{prelude::Collider, spawnable::Spawnable};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

/// A bonus once collided with the snake will increase its size, and thus the
/// player's score.
#[derive(Component, Default, Clone, Copy, Debug)]
pub enum Bonus {
    #[default]
    Normal,
    ExtraBonus,
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

    fn additional_systems(commands: &mut bevy::ecs::system::EntityCommands) {
        commands.insert(Collider);
    }
}
