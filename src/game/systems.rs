// game / system.rs

use std::fs;
use std::thread::current;

use bevy::{animation, prelude::*, transform};
use bevy::window::PrimaryWindow;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};

use crate::components::MyGameCamera;
use crate::game::SimulationState;
use crate::game::resources::AdvanceOneFrameMode;
use crate::resources::MouseCursorWorldCoordinates;
use crate::game::events::*;



use super::components::*;
use super::player::components::*;
use super::resources::DrawnHitboxCoordinates;

// ---------- Structs, Enums, Events ---------- //


// Edit out later
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonTest {
    name: String,
    gender: String,
    age: i32,
}


// ---------- Structs, Enums, Events ---------- //




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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // needs to have access to the current state, and needs to transition to another state
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
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

    if keyboard_input.just_pressed(KeyCode::KeyD) {
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


pub fn level_loader(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    // add something for the csv file
) {
    let window = window_query.get_single().unwrap();
    let loaded_texture: Handle<Image> = asset_server.load("tile-based-game/simplified/level_0/Walls.png");
    let level_transform: Transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);


    let level_entity = commands.spawn(
        SpriteBundle {
            transform: level_transform,
            texture: loaded_texture,
            ..default()
        }
        // needs more components to process the csv file
    );

    // add code to process the csv file /\ /\ look up to the spawn command?
}



/*
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
*/

// system for the camera to follow the player
pub fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<MyGameCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<MyGameCamera>)>,
) {
    let scale_factor: f32 = 0.1;
    for mut camera_transform in camera_query.iter_mut() {
        for player_transform in player_query.iter() {
            let cloned_camera_transform_translation = camera_transform.translation.clone();
            camera_transform.translation += (player_transform.translation - cloned_camera_transform_translation) * scale_factor;
        }
    }
}



// logic isn't entirely right at the moment
// CurrentSpriteSheetIndices
// work on
pub fn animate_sprite(
    time: Res<Time>,
    mut animation_query: Query<(&mut CurrentSpriteSheetIndices, &PlayerSpriteSheetIndices, &mut AnimationTimer, &mut TextureAtlas)>,
    mut animation_start_event_reader: EventReader<AnimationStart>,
    mut animation_end_event_writer: EventWriter<AnimationEnd>,
) {
    for (mut current_sprite_sheet_indices, player_sprite_sheet_indices, mut animation_timer, mut texture_atlas) in animation_query.iter_mut() {
        
        animation_timer.tick(time.delta());

        // If, when looping through events in the start event reader, if we come accross an animation start event, set the current sprite sheet indeces
        for starting_animation_event in animation_start_event_reader.read() {
            if let Some(animation_start_event) = Some(starting_animation_event) {
                current_sprite_sheet_indices.current_first = animation_start_event.starting_index;
                current_sprite_sheet_indices.current_last = animation_start_event.ending_index;
                current_sprite_sheet_indices.looping = animation_start_event.is_looping;
            }
        }

/*
    for event in input_reader.read() {
        if let Some(InputEvent::AttackButtonEvent) = Some(event) {
            println!("Attack Button Event");
            player_attacking_event_writer.send(PlayerAttackEvent);
        }
    }

*/


        if animation_timer.just_finished() { // ie after every 1/60 seconds, ie after every frame
            if texture_atlas.index == current_sprite_sheet_indices.current_last { // <-- if you're at the last index of your current animation
                
                
                // if you're in an animation that loops, don't return to the idle animation
                if current_sprite_sheet_indices.looping {
                    texture_atlas.index = current_sprite_sheet_indices.current_first;


                    // Also send an animation::end event from the current animation
                    animation_end_event_writer.send(AnimationEnd {
                        starting_index: current_sprite_sheet_indices.current_first,
                        ending_index: current_sprite_sheet_indices.current_last,
                        is_looping: true,
                    });

                    println!("fn animate_sprite: texture_atlas_sprite.index: {}", texture_atlas.index);


                }

                // if you're doing anything else, the end of an animation should bring you back to the idle animation
                // this continually runs, figure out?
                if !current_sprite_sheet_indices.looping {

                    // Send an animation::end event
                    animation_end_event_writer.send(AnimationEnd {
                        starting_index: current_sprite_sheet_indices.current_first,
                        ending_index: current_sprite_sheet_indices.current_last,
                        is_looping: false,
                    });

                    // add what to do if the current animation is not a looping one -- we should go back to idle
                    texture_atlas.index = player_sprite_sheet_indices.idle_first;
                    current_sprite_sheet_indices.current_first = player_sprite_sheet_indices.idle_first;
                    current_sprite_sheet_indices.current_last = player_sprite_sheet_indices.idle_last;
                    current_sprite_sheet_indices.looping = false;

                
                    //println!("fn animate_sprite: works works works???");


                }

            } else {
                texture_atlas.index += 1;

                //println!("fn animate_sprite: current index: {}", texture_atlas_sprite.index);
            }
        }
    }
}


// not necessary?
// modify for animations?
// for later -->  https://stackoverflow.com/questions/63675140/how-to-read-bevy-events-without-consuming-them
//I just realized what I was doing wrong. I was using a single global resource EventReader to listen to
//    the JumpedEvent instances being sent. Each EventReader only reads each event a single time.
//    What I needed to do instead was to have an individual EventReader for each system that needed to
//    listen for the event. I did this by using Local EventReaders for each system that needed to listen for the event.
pub fn event_handler(
    mut event_animation_end: EventReader<AnimationEnd>,
) {
    for event in event_animation_end.read() {
        //println!("fn event_handler: Animation End: starting index: {}, ending index: {}", event.starting_index, event.ending_index);

        // do something if in idle -- 0 to 11
        // do something if in atack -- 12 to 18, etc.
        // not sure

    }
}



// ---------- Debug Systems ---------- //

pub fn advance_one_frame(
    mut advance_one_frame_mode: ResMut<AdvanceOneFrameMode>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if simulation_state.get() == &SimulationState::Paused {
        if keyboard_input.just_pressed(KeyCode::KeyZ) {
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    asset_server: Res<AssetServer>,

) {
    if simulation_state.get() == &SimulationState::Paused {
        if keyboard_input.just_pressed(KeyCode::KeyX) {
            
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
    mouse_input: Res<ButtonInput<MouseButton>>,
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

            // send a drawn hitbox event -->
            //   whenever you release a click during draw mode, send drawn hitbox event to a system that checks for those,
            //   then have that system add the current values inside the DrawnHitboxCoordinates resource to a json

        }
    
    }
}


pub fn debug_display_game_resources(
    simulation_state: Res<State<SimulationState>>,
    advance_one_frame_resource: Res<AdvanceOneFrameMode>,
    drawn_hitbox_coordinates: Res<DrawnHitboxCoordinates>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if simulation_state.get() == &SimulationState::Paused {
        if keyboard_input.just_pressed(KeyCode::KeyL) {
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