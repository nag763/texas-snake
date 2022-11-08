use bevy::prelude::{Deref, DerefMut};

#[derive(Default, Deref, DerefMut, Debug, Copy, Clone)]
pub struct Score(pub u32);
