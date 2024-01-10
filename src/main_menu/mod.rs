// main_menu mod.rs
mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use systems::layout::*;
use systems::interactions::*;

use crate::AppState;





pub struct MainMenuPlugin;

// Implements the Plugin trait for MainMenuPlugin struct, which requires the build method to be included
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<MainMenuState>()
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(Update, interaction_button)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)

            ;
    }
}


// I have a transition to the Play and Options states, now I need to despawn all of the Menu elements and spawn the Play or Options elements

// Declaration for Play and Option states (within the Main Menu)
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MainMenuState {
    #[default]
    Menu,
    Play,
    Options,
}
