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


#[derive(Resource)]
pub struct PlayerHitboxData {
    pub idle_box_upper_x: usize,
    pub idle_box_upper_y: usize,
    pub idle_box_lower_x: usize,
    pub idle_box_lower_y: usize,
    pub attack_box_upper_x: usize,
    pub attack_box_upper_y: usize,
    pub attack_box_lower_x: usize,
    pub attack_box_lower_y: usize,
}

impl Default for PlayerHitboxData {
    fn default() -> PlayerHitboxData {
        PlayerHitboxData {
            idle_box_upper_x: 0,
            idle_box_upper_y: 0,
            idle_box_lower_x: 0,
            idle_box_lower_y: 0,
            attack_box_upper_x: 0,
            attack_box_upper_y: 0,
            attack_box_lower_x: 0,
            attack_box_lower_y: 0,
        }
    }
}