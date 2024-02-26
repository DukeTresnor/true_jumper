// game / player / systems.rs


use std::fs;

use bevy::asset::io::file;
use bevy::ecs::entity;
use bevy::input::keyboard;
use bevy::prelude::*;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
//use bevy_animations::*;
use bevy::window::PrimaryWindow;
//use bevy_animations::AnimationDirection;
use crate::game::components::*;
use crate::game::player::components::*;
use crate::game::systems::debug_json_read_write;


use super::events::InputEvent;
// use super::resources::*;
use super::resources::PlayerSpriteSheetData;


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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    player_sprite_sheet_data: Res<PlayerSpriteSheetData>,
    //mut animations: ResMut<Animations>,
) {
    let window = window_query.get_single().unwrap();



    let main_transform_player: Transform = Transform::from_xyz(window.width() / 4.0, window.height() / 2.0, 0.0);
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/ball_blue_large_animation_sheet.png"),
        //Vec2::new(384.0, 80.0), 4, 1, None, None,
        Vec2::new(96.0, 80.0), 12, 2, None, None,
    );

    // Store other texture_atlas's here too
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
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
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(starting_sprite_sheet_indices.current_first),
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
                up_bind: KeyCode::W,
                down_bind: KeyCode::S,
                left_bind: KeyCode::A,
                right_bind: KeyCode::D,
                attack_bind: KeyCode::Space,
            },
            PlayerMovementState {
                is_idle: true,
                is_grounded: false,
                is_walking: false,
                is_dashing: false,
                is_attacking: false,
            },
            PlayerAttackState {
                is_attacking: false,
            },
            starting_sprite_sheet_indices,
            player_sprite_sheet_indices,
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

// don't have a mutable component get the boolean attached output from the keyboard_input resource, instead have
//   a switch case to send events
// Also have an event writer as one of the parameters
pub fn input_handling(
    mut player_query: Query<(&mut PlayerInput, &InputBinding), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
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

pub fn player_movement(
    mut player_query: Query<(&mut Transform, &mut PlayerMovementState), With<Player>>,
    mut input_reader: EventReader<InputEvent>,
) {

    for event in input_reader.read() {
        match event {
            InputEvent::UpEvent=> println!("UpEvent"),
            InputEvent::DownEvent=> println!("DownEvent"), 
            InputEvent::LeftEvent=> println!("LeftEvent"), // <-- do stuff when left event is recieved --> move player to left
            InputEvent::RightEvent=> println!("RightEvent"),
            _=> {}
        }
    }

}

pub fn player_attack(
    // command <-- commands needed to spawn attack / projectile once appropriate input is detected and in proper state.
    mut input_reader: EventReader<InputEvent>,
) {
    for event in input_reader.read() {
        if let Some(InputEvent::AttackButtonEvent) = Some(event) {
            println!("Attack Button Event");
        }
    }
}
