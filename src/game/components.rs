// game / components.rs

use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

// Weird formatting, must be b/c of Deref and DerefMut
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);