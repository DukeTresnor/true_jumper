// game / player / resources.rs


use bevy::prelude::*;



#[derive(Resource)]
pub struct PlayerSpriteSheetData {
    pub idle_first: usize,
    pub idle_last: usize,
    pub attack_first: usize,
    pub attack_last: usize,
}

impl Default for PlayerSpriteSheetData {
    fn default() -> PlayerSpriteSheetData {
        PlayerSpriteSheetData {
            idle_first: 0,
            idle_last: 0,
            attack_first: 0,
            attack_last: 0,
        }
    }
}

