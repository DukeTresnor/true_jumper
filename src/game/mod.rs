// game mod.rs
// create Game plugin
// define SimulationState states -- running and paused
// add systems




// Mod Statements
pub mod resources;
mod systems;
mod components;
pub mod events;
pub mod level_loader;
pub mod player;

// Using
use bevy::prelude::*;
use crate::AppState;
use player::PlayerPlugin;
use level_loader::LevelLoaderPlugin;


use self::{systems::*, resources::*, events::*};


// Constants
pub const GRAVITY: f32 = 1.0;
pub const FRAME_RATE: f32 = 1.0 / 60.0;




// Implementation for GamePlugin. Add any states, systems, plugins, or init any resources that are
//   meant to take place within the game itself (not menus) should go here.
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(AnimationsPlugin {
            //    pixels_per_meter: 20.
            //})
            // .add_systems(OnEnter(AppState::Game), populate_player_hitbox_data.before(spawn_player))
            .init_resource::<AdvanceOneFrameMode>()
            .init_resource::<DrawnHitboxCoordinates>()
            .init_state::<SimulationState>()
            .add_event::<AnimationStart>()
            .add_event::<AnimationEnd>()
            .add_plugins(PlayerPlugin)
            .add_plugins(LevelLoaderPlugin)
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            //.add_systems(OnEnter(AppState::Game), level_loader )

            .add_systems(
                Update,
                (
                    event_handler,
                    animate_sprite,
                    camera_follow_player,
                )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))

            )
            .add_systems(Update, toggle_simulation_state.run_if(in_state(AppState::Game)))
            .add_systems(Update, advance_one_frame.run_if(in_state(AppState::Game)))
            .add_systems(
                Update,
                debug_display_game_resources
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Paused))
            )
            .add_systems(Update, draw_hitbox
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Draw)))
            // Debug systems
            .add_systems(Update, debug_json_read_write.run_if(in_state(AppState::Game)))

            .add_systems(OnExit(AppState::Game), resume_simulation)

            ;
    }
}


// Declaring states for Game plugin -- running, paused
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
    Draw,
}


// Part of code for a system that queries for the player, and assigns components values to variables created for the current scope
// For a single player
//if let Ok((action_state_vector, mut animation_indeces, mut texture_atlas_sprite_sprite_sheet)) = player_query.get_single_mut() {