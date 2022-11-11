pub mod border_set;
pub mod font {
    use bevy::prelude::{Deref, DerefMut, Font, Handle};
    /// The app font, loaded on startup and returned as a ressource.
    #[derive(Deref, DerefMut, Debug, Clone, Default)]
    pub struct AppFont(Option<Handle<Font>>);
}
pub mod game_state;
pub mod score {
    use bevy::prelude::{Deref, DerefMut};
    /// The score equals the snake length, and defines the
    /// user progress in the game.
    #[derive(Default, Deref, DerefMut, Debug, Copy, Clone)]
    pub struct Score(pub u32);
}

pub mod prelude {
    pub use super::border_set::*;
    pub use super::font::AppFont;
    pub use super::game_state::GameState;
    pub use super::score::Score;
}
