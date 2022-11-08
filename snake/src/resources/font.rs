use bevy::prelude::{Deref, DerefMut, Font, Handle};

#[derive(Deref, DerefMut, Debug, Clone, Default)]
pub struct AppFont(Option<Handle<Font>>);
