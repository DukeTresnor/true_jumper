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


pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    // resource to access current state
    app_state: Res<State<AppState>>,
    // mutable resource to access the next app state
    mut next_app_state: ResMut<NextState<AppState>>,

) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.get() != &AppState::Game {
            next_app_state.set(AppState::Game);
            println!("I transitioned to the game state");
        }

    }
    
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    // resource to access current state
    app_state: Res<State<AppState>>,
    // mutable resource to access the next app state
    mut next_app_state: ResMut<NextState<AppState>>,
    // mutable resource to access the next simulation state
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.get() != &AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_simulation_state.set(SimulationState::Paused);
            println!("I transitioned to the main menu state");
        }
    }
}



pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
        println!("I exited the game and closed the window");
    }
}