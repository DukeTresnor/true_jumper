use bevy::prelude::*;
//use bevy_ecs_ldtk::prelude::*;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use resources::MouseCursorWorldCoordinates;
use systems::*;


mod main_menu;
mod game;
mod systems;
mod components;
mod resources;




fn main() {
    App::new()
        .init_resource::<MouseCursorWorldCoordinates>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        //.add_plugins(LdtkPlugin)
        .init_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, transition_to_game_state)     
        .add_systems(Update, transition_to_main_menu_state)
        .add_systems(Update, populate_mouse_cursor_world_coordinates)
        .add_systems(Update, exit_game)
        .run()

}


// Declaration for MainMenu and Game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}