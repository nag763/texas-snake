pub mod snake;

pub mod border {
    use bevy::prelude::*;
    /// A border is a non traversable component that will result in a game over once collided.
    #[derive(Debug, Default, Component)]
    pub struct Border;
}

pub mod collider {
    use bevy::prelude::*;
    /// A collider is something the snake can't go through without triggering an event.
    ///
    /// It can either be a bonus, or a border.
    #[derive(Debug, Default, Component)]
    pub struct Collider;
}

pub mod bonus;

pub mod spawnable;

pub mod queue;

pub mod prelude {
    pub use super::bonus::Bonus;
    pub use super::border::*;
    pub use super::collider::*;
    pub use super::queue::*;
    pub use super::snake::*;
    pub use super::spawnable::Spawnable;
}
