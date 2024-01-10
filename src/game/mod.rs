// game mod.rs
// create Game plugin
// define SimulationState states -- running and paused
// add systems


// Constants
pub const  GRAVITY: f32 = 1.0;
pub const FRAME_RATE: f32 = 1.0 / 60.0;


use bevy::prelude::*;

// Implementation for GamePlugin. Add any states, systems, plugins, or init any resources that are
//   meant to take place within the game itself (not menus) should go here.
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SimulationState>()
            ;
    }
}


// Declaring states for Game plugin -- running, paused
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}


// Part of code for a system that queries for the player, and assigns components values to variables created for the current scope
// For a single player
//if let Ok((action_state_vector, mut animation_indeces, mut texture_atlas_sprite_sprite_sheet)) = player_query.get_single_mut() {