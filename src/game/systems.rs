// game / system.rs

use bevy::prelude::*;

use crate::game::SimulationState;
use crate::game::resources::AdvanceOneFrameMode;


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
        }

    } else if simulation_state.get() == &SimulationState::Running && advance_one_frame_mode.should_advance_one_frame {
        next_simulation_state.set(SimulationState::Paused);
        advance_one_frame_mode.should_advance_one_frame = false;
    } 
}