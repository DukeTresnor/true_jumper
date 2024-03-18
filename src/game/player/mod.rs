// game / player / mod.rs
// define player states

// Mod Statements
pub mod resources;
mod systems;
pub mod components;
pub mod events;

// Using
use bevy::prelude::*;
use crate::AppState;
use systems::*;

use self::{events::{InputEvent, PlayerAttackEvent, PlayerWalkingEvent}, resources::{PlayerHitboxData, PlayerSpriteSheetData}};

use super::{SimulationState, resources::DrawnHitboxCoordinates};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<MovementState>()
            .init_state::<AirState>()
            .init_state::<AttackState>()
            .init_resource::<PlayerSpriteSheetData>()
            .init_resource::<PlayerHitboxData>()
            .add_event::<InputEvent>()
            .add_event::<PlayerWalkingEvent>()
            .add_event::<PlayerAttackEvent>()
            .add_systems(OnEnter(AppState::Game), populate_player_sprite_sheet_indeces.before(spawn_player))
            .add_systems(OnEnter(AppState::Game), populate_player_hitbox_data.before(spawn_player))
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(Update, 
                (
                    input_handling,
                    player_movement,
                    player_attack,
                    player_animation_setter,
                )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(AppState::Game), despawn_player)
        ;
    }
}


// .init_resource::<AdvanceOneFrameMode>()


//Possibly use these in the future,  but for now keep state changes to things outside of the player, and use components for keeping track of what the player is doing
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MovementState {
    #[default]
    Idle,
    Walking,
    Dashing,
    Falling,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AirState {
    #[default]
    Airborne,
    Grounded,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AttackState {
    #[default]
    Neutral,
    Attacking
}
