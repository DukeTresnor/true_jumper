// game / player / components.rs

use bevy::prelude::*;



#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub attack: bool,
}

#[derive(Component)]
pub struct  InputBinding {
    pub up_bind: KeyCode,
    pub down_bind: KeyCode,
    pub left_bind: KeyCode,
    pub right_bind: KeyCode,
    pub attack_bind: KeyCode,
}

#[derive(Component)]
pub struct PlayerMovementState {
    pub is_grounded: bool,
    pub is_walking: bool,
    pub is_dashing: bool,
    pub is_attacking: bool,
}

#[derive(Component)]
pub struct PlayeraAttackState {
    pub is_attacking: bool,
}


#[derive(Component)]
pub struct CurrentSpriteSheetIndices {
    pub current_first: usize,
    pub current_last: usize,
    pub looping: bool,
}

#[derive(Component)]
pub struct PlayerSpriteSheetIndices {
    pub idle_first: usize,
    pub idle_last: usize,
    pub attack_first: usize,
    pub attack_last: usize,
    // add more as you add animations to the spritesheet
}