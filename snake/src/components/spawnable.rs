use bevy::{
    asset::Assets,
    ecs::bundle::Bundle,
    ecs::component::Component,
    ecs::system::{Commands, EntityCommands, ResMut},
    render::mesh::Mesh,
    sprite::ColorMaterial,
    transform::components::Transform,
};

/// The spawnable trait is a trait used to make easier the spawn of components.
///
/// It allows overall to rather stock the spawn mechanism along the source code of the component
/// rather than the systems.
pub trait Spawnable<T>
where
    Self: Component + Copy,
    T: Bundle,
{
    /// Returns the bundle of the component.
    fn get_bundle(
        &self,
        transform: Transform,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> T;

    /// Spawns the component.
    fn spawn(
        &self,
        position: Transform,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        let bundle: T = Self::get_bundle(self, position, materials, meshes);
        let mut commands = commands.spawn();
        commands.insert(*self).insert_bundle(bundle);
        Self::additional_systems(self, &mut commands);
    }

    /// Additional systems, can be handful if for instance,
    /// there is a need to add another component to the bundle.
    fn additional_systems(&self, commands: &mut EntityCommands);
}
