// game / player / components.rs

use bevy::prelude::*;

// Temporary Constants -- add json file to supply this data later
// ------ //
pub const PLAYER_HEALTH: f32 = 10.0;
pub const PLAYER_HORIZONTAL_SPEED: f32 = 50.0;
pub const PLAYER_VERTICAL_SPEED: f32 = 10.0;


// ------ //


#[derive(Component)]
pub struct Player {}


// Remove once you implement events
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

// --------- //
// might not be necessary anymore
#[derive(Component)]
pub struct PlayerMovementState {
    pub is_idle: bool,
    pub is_grounded: bool,
    pub is_walking: bool,
    pub is_dashing: bool,
    pub is_attacking: bool,
}

#[derive(Component)]
pub struct PlayerAttackState {
    pub is_attacking: bool,
}

// --------- //



#[derive(Component)]
pub struct CurrentSpriteSheetIndices {
    pub current_first: usize,
    pub current_last: usize,
    pub looping: bool,
}

// Components that store data from imported json file
// -------- //
#[derive(Component)]
pub struct PlayerSpriteSheetIndices {
    pub idle_first: usize,
    pub idle_last: usize,
    pub attack_first: usize,
    pub attack_last: usize,
    // add more as you add animations to the spritesheet and the json file
}

// import logic not created yet, uses constants defined above
#[derive(Component)]
pub struct PlayerStats {
    pub player_health: f32,
    pub player_horizontal_speed: f32,
    pub player_vertical_speed: f32,
}

// -------- //