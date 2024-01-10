// main systems
// spawn a camera
//     needs a window and commands
// handle main transitions:
//     transition to game state
//     transition to main menu state
// exit game
// handle game over event





// use statements

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::app::AppExit;

use crate::AppState;
use crate::game::SimulationState;
use crate::components::*;


// Systems
pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (
            Camera2dBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 500.0),
                ..default()
            },
            MyGameCamera {},
        )
    );
}