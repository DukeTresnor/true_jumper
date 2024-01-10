use bevy::prelude::*;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;


mod main_menu;
mod game;
mod systems;
mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)        

        .run()

}


// Declaration for MainMenu and Game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}