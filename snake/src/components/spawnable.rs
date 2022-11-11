use bevy::{
    asset::Assets,
    ecs::bundle::Bundle,
    ecs::component::Component,
    ecs::system::{Commands, EntityCommands, ResMut},
    render::mesh::Mesh,
    sprite::ColorMaterial,
    transform::components::Transform,
};

pub trait Spawnable<T>
where
    Self: Component + Copy,
    T: Bundle,
{
    /// A bonus once collide
    fn get_bundle(
        &self,
        transform: Transform,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> T;

    fn spawn(
        &self,
        position: Transform,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        let bundle: T = Self::get_bundle(&self, position, materials, meshes);
        let mut commands = commands.spawn();
        commands.insert(self.clone()).insert_bundle(bundle);
        Self::additional_systems(&mut commands);
    }

    fn additional_systems(commands: &mut EntityCommands);
}
