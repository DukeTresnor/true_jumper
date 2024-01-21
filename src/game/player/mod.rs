// game / player / mod.rs
// define player states

// Mod Statements
pub mod resources;
mod systems;
mod components;

// Using
use bevy::prelude::*;
use crate::AppState;
use systems::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_player)
        ;
    }
}