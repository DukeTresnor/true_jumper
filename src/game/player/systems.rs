// game / player / systems.rs


use std::{default, fs};
use std::thread::current;

use bevy::asset::io::file;
use bevy::ecs::entity;
use bevy::input::keyboard;
use bevy::prelude::*;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
//use bevy_animations::*;
use bevy::window::PrimaryWindow;
//use bevy_animations::AnimationDirection;
use crate::game::{components::*, AnimationEnd, AnimationStart};
use crate::game::player::components::*;
use crate::game::systems::debug_json_read_write;


use super::events::*;
// use super::resources::*;
use super::resources::PlayerSpriteSheetData;
use super::{AirState, MovementState};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct JsonSpriteSheet {
    idle_first: usize,
    idle_last: usize,
    attack_first: usize,
    attack_last: usize,
}

// have a system that upon entering AppState Game reads json info from a json file, and stores it into a resource that the spawn player system can pull from
// resource is initilized with dummy info?
pub fn populate_player_sprite_sheet_indeces(
    mut player_sprite_sheet_data: ResMut<PlayerSpriteSheetData>,
) {

    // path to json data
    let path = "/Users/bradfordarmstrong/Projects/rust_space/bevy_games/true_jumper/src/game/player/json_data/player_sprite_sheet_indices.json";

    let file_data = fs::read_to_string(path)
        .expect("Unable to read file");

    let json_data: JsonSpriteSheet = serde_json::from_str(&file_data)
        .expect("Not the correct format -----");


    //player_sprite_sheet_data.idle_first = json_data.idle_first;

    let my_data: JsonSpriteSheet = json_data.clone();

    player_sprite_sheet_data.idle_first = my_data.idle_first;
    player_sprite_sheet_data.idle_last = my_data.idle_last;
    player_sprite_sheet_data.attack_first = my_data.attack_first;
    player_sprite_sheet_data.attack_last = my_data.attack_last;


    //println!("fn populate_player_sprite_sheet_indeces: my_data: {:?}", my_data);
    
    //println!("fn populate_player_sprite_sheet_indeces: idle_first: {}", json_data.idle_first);
    //println!("fn populate_player_sprite_sheet_indeces: player_sprite_sheet_data.idle_first: {}", player_sprite_sheet_data.idle_first);

}



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct JsonHitboxData {
    idle_box_upper_x: usize,
    idle_box_upper_y: usize,
    idle_box_lower_x: usize,
    idle_box_lower_y: usize,
    attack_box_upper_x: usize,
    attack_box_upper_y: usize,
    attack_box_lower_x: usize,
    attack_box_lower_y: usize,
}

// this system should work similarly to populate_player_sprite_sheet_indeces, in that it should pull data from a json file
//   and populate the player with the proper hitbox data
pub fn populate_player_hitbox_data(

) {

}


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_sprite_sheet_data: Res<PlayerSpriteSheetData>,
    //mut animations: ResMut<Animations>,
) {
    let window = window_query.get_single().unwrap();
    let loaded_texture = asset_server.load("sprites/ball_blue_large_animation_sheet.png");
    let main_transform_player: Transform = Transform::from_xyz(window.width() / 4.0, window.height() / 2.0, 0.1);
    let texture_atlas = TextureAtlasLayout::from_grid(
        Vec2::new(96.0, 80.0), 17, 2, None, None,
    );

    // Store other texture_atlas's here too
    let texture_atlas_layouts_handle = texture_atlas_layouts.add(texture_atlas);
    //let cloned_texture_atlas_handle = texture_atlas_handle.clone();
    //let idle_animation_indeces = AnimationIndices { first: 0, last: 3 };



    let player_sprite_sheet_indices = PlayerSpriteSheetIndices {
        idle_first: player_sprite_sheet_data.idle_first,
        idle_last: player_sprite_sheet_data.idle_last,
        attack_first: player_sprite_sheet_data.attack_first,
        attack_last: player_sprite_sheet_data.attack_last,
    };

    let starting_sprite_sheet_indices = CurrentSpriteSheetIndices {
        current_first: player_sprite_sheet_indices.idle_first,
        current_last: player_sprite_sheet_indices.idle_last,
        looping: true,
    };


    // Spawning Player entity
    let player_entity = commands.spawn(
        (
            SpriteSheetBundle {
                transform: main_transform_player,
                texture: loaded_texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layouts_handle,
                    index: 0
                },
                //sprite: TextureAtlasSprite::new(starting_sprite_sheet_indices.current_first),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(1.0 / 60.0, TimerMode::Repeating)),
            Player {},
            PlayerInput {
                up: false,
                down: false,
                left: false,
                right: false,
                attack: false,
            },
            InputBinding {
                up_bind: KeyCode::KeyW,
                down_bind: KeyCode::KeyS,
                left_bind: KeyCode::KeyA,
                right_bind: KeyCode::KeyD,
                attack_bind: KeyCode::Space,
            },
            PlayerMovementState {                  // <-- this component I think is not necessary -- test removing it eventually
                is_idle: true,
                is_grounded: false,
                is_walking: false,
                is_dashing: false,
                is_attacking: false,
            },
            PlayerAttackState {                    // <-- same with this component
                is_attacking: false,
            },
            starting_sprite_sheet_indices,
            player_sprite_sheet_indices,
            PlayerStats {
                player_health: PLAYER_HEALTH,
                player_horizontal_speed: PLAYER_HORIZONTAL_SPEED,
                player_vertical_speed: PLAYER_VERTICAL_SPEED,
            }
        )
    );

    /* 
    // Adding Animations
    animations.insert_animation(
        player_entity.id(),
        AnimationType::Transform(
            TransformAnimation::new(
                vec![0, 1, 2, 3],
                0.55,
                cloned_texture_atlas_handle,
                Vec2::new(0.0, 0.0),
                AnimationDirectionIndexes::IndexBased(IndexBasedDirection {
                    left: 1,
                    right: 1,
                    up: 1,
                    down: 1
                }),
                true
            ), 
            "player_idle"
        )
    );
    */
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    //player_query: Query<(Entity, &Player)>,
) {
    // If player_entity exists because the player_query contains some entity with the Player component,
    //   we want to despawn that player entity with commands
    //if let Ok(player_entity) = player_query.get_single() {
    for player_entity in player_query.iter() {
        commands.entity(player_entity).despawn()
    }
}


// don't have a mutable component get the boolean attached output from the keyboard_input resource, instead have
//   a switch case to send events
// Also have an event writer as one of the parameters
pub fn input_handling(
    mut player_query: Query<(&mut PlayerInput, &InputBinding), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // Controller input -- controller_input: Res<Input< Controller Code ?? >>,
    mut input_event: EventWriter<InputEvent>,

) {
    for (mut player_input, input_binding) in player_query.iter_mut() {
        player_input.up = keyboard_input.pressed(input_binding.up_bind);
        player_input.down = keyboard_input.pressed(input_binding.down_bind);
        player_input.left = keyboard_input.pressed(input_binding.left_bind);
        player_input.right = keyboard_input.pressed(input_binding.right_bind);
        player_input.attack = keyboard_input.just_pressed(input_binding.attack_bind);


        // not sure if proper structure, can I use a switch case with match?
        if keyboard_input.pressed(input_binding.up_bind) {
            input_event.send(InputEvent::UpEvent);
        }
    
        if keyboard_input.pressed(input_binding.down_bind) {
            input_event.send(InputEvent::DownEvent);
        }
        
        if keyboard_input.pressed(input_binding.left_bind) {
            input_event.send(InputEvent::LeftEvent);
        }

        if keyboard_input.pressed(input_binding.right_bind) {
            input_event.send(InputEvent::RightEvent);
        }

        if keyboard_input.just_pressed(input_binding.attack_bind) {
            input_event.send(InputEvent::AttackButtonEvent);
        }

    }



/*
          // Also send an animation::end event
                event_animation_end.send(AnimationEnd {
                    starting_index: current_sprite_sheet_indices.current_first,
                    ending_index: current_sprite_sheet_indices.current_last,
                });


*/


}






// constantly checks for the ending of an animation for an attack / projectile that comes from the player
pub fn despawn_children(
    // command <-- should take command as a parameter since we are intending to despawn
) {

}

/* 
pub fn move_player(
    mut player_query: Query<(Entity, &mut Transform, &PlayerInput, &mut PlayerMovementState), With<Player>>,
    mut event_writer: EventWriter<AnimationEvent>,
) {
    for (player_entity, mut player_transform, player_input, mut player_movement_state) in player_query.iter_mut() {
        // move logic
        // sending event
        event_writer.send(AnimationEvent("player_idle", player_entity));   
    }
}
*/

// ---- DELETE ---- //
pub fn temp_up_down_move_player(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &PlayerStats), With<Player>>,
    mut input_reader: EventReader<InputEvent>,
) {
    for (mut player_transform, player_stats) in player_query.iter_mut() {
        for event in input_reader.read() {
            let mut direction = Vec3::ZERO;
            match event {
                InputEvent::UpEvent => {
                    direction += Vec3::new(0.0, 1.0, 0.0);
                    if direction.length() > 0.0 {
                        direction = direction.normalize();
                    }
                    player_transform.translation += direction * player_stats.player_horizontal_speed * time.delta_seconds();
                },
                InputEvent::DownEvent => {
                    direction += Vec3::new(0.0, -11.0, 0.0);
                    if direction.length() > 0.0 {
                        direction = direction.normalize();
                    }
                    player_transform.translation += direction * player_stats.player_horizontal_speed * time.delta_seconds();
                },
                _=> {}
            }
        }
    }
}
// ---- DELETE ---- //

pub fn player_movement(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &PlayerStats), With<Player>>,
    mut animation_end_event_reader: EventReader<AnimationEnd>,
    mut input_reader: EventReader<InputEvent>,
    mut player_walking_event_writer: EventWriter<PlayerWalkingEvent>,
    movement_state: Res<State<MovementState>>,
    mut next_movement_state: ResMut<NextState<MovementState>>,
    air_state: Res<State<AirState>>,
) {

    for (mut player_transform, player_stats) in player_query.iter_mut() {
        for event in input_reader.read() {
            let mut direction = Vec3::ZERO;
            match event {
                InputEvent::UpEvent=> {
                    println!("UpEvent");
                    if air_state.get() == &AirState::Grounded {
                        // give some vertical force movement, send animation start event
                    }
                },
                InputEvent::DownEvent=> println!("DownEvent"), 
                InputEvent::LeftEvent=> {
                    println!("LeftEvent");
                    if (movement_state.get() == &MovementState::Idle) | (movement_state.get() == &MovementState::Walking) {
                        // move player entity to the left
                        direction += Vec3::new(-1.0, 0.0, 0.0);
                        if direction.length() > 0.0 {
                            direction = direction.normalize();
                        }
                        // transform.translation += direction * PLAYER_SPEED_HORIZONTAL * time.delta_seconds();
                        player_transform.translation += direction * player_stats.player_horizontal_speed * time.delta_seconds();
                        // set movement state to walking, but only if the player is not walking -- to not sent repetitive events
                        if !(movement_state.get()  == &MovementState::Walking) {
                            next_movement_state.set(MovementState::Walking);
                        }
                        // send walking event
                        player_walking_event_writer.send(PlayerWalkingEvent{walking_direction: direction});
                    }
                }, // <-- do stuff when left event is recieved --> move player to left
                InputEvent::RightEvent=> {
                    println!("RightEvent");
                    if (movement_state.get() == &MovementState::Idle) | (movement_state.get() == &MovementState::Walking) {
                        // move player entity to the left
                        direction += Vec3::new(1.0, 0.0, 0.0);
                        if direction.length() > 0.0 {
                            direction = direction.normalize();
                        }
                        // transform.translation += direction * PLAYER_SPEED_HORIZONTAL * time.delta_seconds();
                        player_transform.translation += direction * player_stats.player_horizontal_speed * time.delta_seconds();
                        // set movement state to walking, but only if the player is not walking -- to not sent repetitive events
                        if !(movement_state.get()  == &MovementState::Walking) {
                            next_movement_state.set(MovementState::Walking);
                        }
                        // send walking event
                        player_walking_event_writer.send(PlayerWalkingEvent{walking_direction: direction});
                    }
                },
                _=> {}
            }
        }
        

        // portion of system to change state back to idle
        for animation_end_event in animation_end_event_reader.read() {
            // if the animation end event is a looping one, don't change state, but if it isn't looping, change back to idle
            if !animation_end_event.is_looping {
                next_movement_state.set(MovementState::Idle);
            }
        }
    }



}

pub fn player_attack(
    // command <-- commands needed to spawn attack / projectile once appropriate input is detected and in proper state.
    mut player_query: Query<&mut PlayerAttackState, With<Player>>,
    mut input_reader: EventReader<InputEvent>,
    mut player_attack_event_writer: EventWriter<PlayerAttackEvent>,
    mut animation_end_event_reader: EventReader<AnimationEnd>,
) {

    for mut player_attack_state in player_query.iter_mut() {
        for event in input_reader.read() {
            if let Some(InputEvent::AttackButtonEvent) = Some(event) {
                println!("Attack Button Event");
                player_attack_event_writer.send(PlayerAttackEvent);

                // switch player state to attacking
                if !player_attack_state.is_attacking {
                    player_attack_state.is_attacking = true;
                }

            }
        }

        // When an animation ends, if we're in an attacking state, exit the attacking state
        for animation_end_event in animation_end_event_reader.read() {
            if let Some(real_animation_end_event) = Some(animation_end_event) {
                if player_attack_state.is_attacking {
                    println!("fn player_attack: Attack animation event ended");
                    player_attack_state.is_attacking = false;
                }




            }
        }

    }

}


pub fn player_animation_setter(
    mut player_query: Query<(&PlayerSpriteSheetIndices, &mut CurrentSpriteSheetIndices, &PlayerAttackState), With<Player>>,
    mut player_walking_event_reader: EventReader<PlayerWalkingEvent>,
    mut player_attacking_event_reader: EventReader<PlayerAttackEvent>,
    mut animation_start_event_writer: EventWriter<AnimationStart>,
) {


    for (player_sprite_sheet_indeces, mut current_sprite_sheet_indecs, player_attack_state) in player_query.iter_mut() {
        // Process walking
        for walking_event in player_walking_event_reader.read() {
            if let Some(player_walking_event) = Some(walking_event) {
                
                if player_walking_event.walking_direction.x == -1.0 {
                    println!("I'm walking left");
                    // send an animation start event for walking left
                }
                
                if player_walking_event.walking_direction.x == 1.0 {
                    println!("I'm walking right");
                    // send an animation start event for walking right
                }

                /* send an animation start event for walking
                animation_start_event_writer.send(
                    AnimationStart {
                        starting_index
                        ending_index
                        is_looping
                    }
                )                
                 */


            }

            //use this for detecting if you're walking left or right in order to load the
            //  proper animation --> println!("walking is: {}", walking_event.walking_direction)
            // set animation indeces to values for walking
        }

        // Process attacking
        for attacking_event in player_attacking_event_reader.read() {

            println!("I'm attacking");
            // set your current indeces to the indeces meant for attacking -- attackFirst and attackLast
            current_sprite_sheet_indecs.current_first = player_sprite_sheet_indeces.attack_first;
            current_sprite_sheet_indecs.current_last = player_sprite_sheet_indeces.attack_last;
            
            //spawn a child entity using the attack animation sheet

            // send an animation start event
            //   right now is_looping should be set to something like player_sprite_sheet_indeces.attack_is_looping,
            //     but I currently don't have that data present within the sprite sheet data...
            animation_start_event_writer.send(
                AnimationStart {
                    starting_index: current_sprite_sheet_indecs.current_first,
                    ending_index: current_sprite_sheet_indecs.current_last,
                    is_looping: false,
                }
            );
            //println!("fn player_animation_setter: sdkfjnvbfvhbdjhbdhjdfbjhdfbsssssssssssssd");
        }
    }



}