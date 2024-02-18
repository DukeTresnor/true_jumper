// game / system.rs

use std::fs;
use std::thread::current;

use bevy::{prelude::*, animation};
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};

use crate::game::SimulationState;
use crate::game::resources::AdvanceOneFrameMode;
use crate::resources::MouseCursorWorldCoordinates;



use super::components::*;
use super::player::components::*;
use super::resources::DrawnHitboxCoordinates;

// Edit out later
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

    if keyboard_input.just_pressed(KeyCode::D) {
        if simulation_state.get() == &SimulationState::Paused {
            // Enter Draw state
            next_simulation_state.set(SimulationState::Draw);
            println!("Entering Draw State");
        }
        if simulation_state.get() == &SimulationState::Draw {
            // Return to Paused State
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation return to Paused");
        }
    }

}

// CurrentSpriteSheetIndices
// work on
pub fn animate_sprite(
    time: Res<Time>,
    mut animation_query: Query<(&CurrentSpriteSheetIndices, &PlayerSpriteSheetIndices, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (current_sprite_sheet_indices, player_sprite_sheet_indices, mut animation_timer, mut texture_atlas_sprite) in animation_query.iter_mut() {
        animation_timer.tick(time.delta());



        if animation_timer.just_finished() { // ie after every 1/60 seconds, ie after every frame
            if texture_atlas_sprite.index == current_sprite_sheet_indices.current_last {
                
                
                // if you're walking, you should loop walking animation
                if current_sprite_sheet_indices.looping {
                    texture_atlas_sprite.index = current_sprite_sheet_indices.current_first;
                }

                // if you're doing anything else, the end of an animation should bring you back to the idle animation
                if !current_sprite_sheet_indices.looping {
                    // add what to do if the current animation is not a looping one -- we should go back to idle
                    texture_atlas_sprite.index = player_sprite_sheet_indices.idle_first;
                }
                

                // Also send an animation::end event

            } else {
                texture_atlas_sprite.index += 1;

                //println!("fn animate_sprite: current index: {}", texture_atlas_sprite.index);
            }
        }
    }
}




// ---------- Debug Systems ---------- //

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


// Refine, rename
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


// needs to send an event
pub fn draw_hitbox(
    cursor_coordinates: Res<MouseCursorWorldCoordinates>,
    mouse_input: Res<Input<MouseButton>>,
    mut drawn_hitbox_coordinates: ResMut<DrawnHitboxCoordinates>,
    player_query: Query<&Transform, With<Player>>,
) {

    if mouse_input.just_pressed(MouseButton::Left) {
        drawn_hitbox_coordinates.starting_coordinates.x = cursor_coordinates.0.x;
        drawn_hitbox_coordinates.starting_coordinates.y = cursor_coordinates.0.y;
    }

    if mouse_input.just_released(MouseButton::Left) {
        drawn_hitbox_coordinates.ending_coordinates.x = cursor_coordinates.0.x;
        drawn_hitbox_coordinates.ending_coordinates.y = cursor_coordinates.0.y;

        //println!("fn draw_hitbox: starting_box_coordinates x: {}, starting_box_coordinates y: {}", drawn_hitbox_coordinates.starting_coordinates.x, drawn_hitbox_coordinates.starting_coordinates.y);
        //println!("fn draw_hitbox: ending_box_coordinates x: {}, ending_box_coordinates y: {}", drawn_hitbox_coordinates.ending_coordinates.x, drawn_hitbox_coordinates.ending_coordinates.y);
    

        for player_transform in player_query.iter() {
            println!("fn draw_hitbox: player_transform: x_coordinate: {}, y_coordinate: {}", player_transform.translation.x, player_transform.translation.y);
        
            // calculate the x and y coordinates of the starting and ending positions of the drawn hitboxes relative to the player's position
            drawn_hitbox_coordinates.starting_coordinates_relative_to_player.x = drawn_hitbox_coordinates.starting_coordinates.x - player_transform.translation.x;
            drawn_hitbox_coordinates.starting_coordinates_relative_to_player.y = drawn_hitbox_coordinates.starting_coordinates.y - player_transform.translation.y;
            drawn_hitbox_coordinates.ending_coordinates_relative_to_player.x = drawn_hitbox_coordinates.ending_coordinates.x - player_transform.translation.x;
            drawn_hitbox_coordinates.ending_coordinates_relative_to_player.y = drawn_hitbox_coordinates.ending_coordinates.y - player_transform.translation.y;

            // send a drawn hitbox event

        }
    
    }
}


pub fn debug_display_game_resources(
    simulation_state: Res<State<SimulationState>>,
    advance_one_frame_resource: Res<AdvanceOneFrameMode>,
    drawn_hitbox_coordinates: Res<DrawnHitboxCoordinates>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if simulation_state.get() == &SimulationState::Paused {
        if keyboard_input.just_pressed(KeyCode::L) {
            println!("fn debug_display_game_resources: should_advance_one_frame: {}, frame_timer: {:?}", advance_one_frame_resource.should_advance_one_frame, advance_one_frame_resource.frame_timer);
            println!("fn debug_display_game_resources: starting_coordinates: {}, ending_coordinates: {}, starting_coordinates_relative_to_player: {}, ending_coordinates_relative_to_player: {}", 
                drawn_hitbox_coordinates.starting_coordinates, 
                drawn_hitbox_coordinates.ending_coordinates, 
                drawn_hitbox_coordinates.starting_coordinates_relative_to_player, 
                drawn_hitbox_coordinates.ending_coordinates_relative_to_player);
        }
    }
}



// ---------- Debug Systems ---------- //