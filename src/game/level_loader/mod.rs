// game / level_loader / mod.rs



// Mod Statements
pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

// Using statements
use bevy::prelude::*;
use crate::AppState;


use self::{systems::*, resources::*, events::*};


pub struct LevelLoaderPlugin;
impl Plugin for LevelLoaderPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_systems(OnEnter(AppState::Game), level_loader)
        
            ;
    }
}