// game / system.rs

use std::fs;

use bevy::prelude::*;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};

use crate::game::SimulationState;
use crate::game::resources::AdvanceOneFrameMode;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonTest {
    name: String,
    gender: String,
    age: i32,
}


pub fn resume_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>
) {
    next_simulation_state.set(SimulationState::Running);
    println!("I resumed the simulation")
}


pub fn pause_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>
) {
    next_simulation_state.set(SimulationState::Paused);
    println!("I paused the simulation")
}

pub fn toggle_simulation_state(
    // needs access to keyboard input
    keyboard_input: Res<Input<KeyCode>>,
    // needs to have access to the current state, and needs to transition to another state
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        if simulation_state.get() == &SimulationState::Paused {
            // Enter Running with set()
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation Running");
        }
        if simulation_state.get() == &SimulationState::Running {
            // Enter Paused with set()
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation Paused");
        }
    }
}

pub fn advance_one_frame(
    mut advance_one_frame_mode: ResMut<AdvanceOneFrameMode>,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if simulation_state.get() == &SimulationState::Paused {
        if keyboard_input.just_pressed(KeyCode::Z) {
            advance_one_frame_mode.should_advance_one_frame = true; 
            next_simulation_state.set(SimulationState::Running);
            println!("Advance kjnfkjdfn");
        }

    } else if simulation_state.get() == &SimulationState::Running && advance_one_frame_mode.should_advance_one_frame {
        next_simulation_state.set(SimulationState::Paused);
        advance_one_frame_mode.should_advance_one_frame = false;
    } 
}

// ---------- Debug Systems ---------- //

pub fn debug_json_read_write(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    asset_server: Res<AssetServer>,

) {
    if simulation_state.get() == &SimulationState::Paused {
        if keyboard_input.just_pressed(KeyCode::X) {
            
            // load json data into a string
            let data = fs::read_to_string("/Users/bradfordarmstrong/Projects/rust_space/bevy_games/true_jumper/src/game/json_test.json")
                .expect("Unable to read file");

            // convert data into a JsonTest struct
            let mut json_test_num_inf: JsonTest = serde_json::from_str(&data).expect("Not the correct format -----");

            // change values
            json_test_num_inf.age = 54;

            // convert JsonTest to a string
            let stringified_json_test_num_inf = serde_json::to_string(&json_test_num_inf).expect("Wrong structure?");

            // write to original file
            let _ = fs::write("/Users/bradfordarmstrong/Projects/rust_space/bevy_games/true_jumper/src/game/json_test.json", stringified_json_test_num_inf);

        }

    }
}



// ---------- Debug Systems ---------- //