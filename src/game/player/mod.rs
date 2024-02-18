// game / player / mod.rs
// define player states

// Mod Statements
pub mod resources;
mod systems;
pub mod components;

// Using
use bevy::prelude::*;
use crate::AppState;
use systems::*;

use self::resources::{PlayerSpriteSheetData, PlayerHitboxData};

use super::{SimulationState, resources::DrawnHitboxCoordinates};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_state::<MovementState>()
            //.add_state::<AttackState>()
            .init_resource::<PlayerSpriteSheetData>()
            .init_resource::<PlayerHitboxData>()
            .add_systems(OnEnter(AppState::Game), populate_player_sprite_sheet_indeces.before(spawn_player))
            .add_systems(OnEnter(AppState::Game), populate_player_hitbox_data.before(spawn_player))
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(Update, 
                (
                    input_handling,
                    //move_player
                )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            )
        ;
    }
}


// .init_resource::<AdvanceOneFrameMode>()


/*     Possibly use these in the future,  but for now keep state changes to things outside of the player, and use components for keeping track of what the player is doing
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MovementState {
    #[default]
    Idle,
    Walking,
    Dashing,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AttackState {
    #[default]
    Neutral,
    Attacking
}
*/