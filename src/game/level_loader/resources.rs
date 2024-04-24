// game / level_loader / systems.rs

use bevy::prelude::*;

/*
        structure for Level Resource:
            level_values: Vec<[f32, f32]>,
            rows: i32,
            columns: i32,

*/

#[derive(Resource)]
pub struct LevelData {
    pub cell_values: Vec<i32>,
    pub row_number: i32,
    pub col_number: i32,
}

impl Default for LevelData {
    fn default() -> LevelData {
        LevelData {
            cell_values: Vec::new(),
            row_number: 0,
            col_number: 0,
        }
    }
}