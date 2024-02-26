// game / player / events.rs

use bevy::prelude::*;


/*

// should these events go into an events module? probably
#[derive(Event)]
pub struct AnimationEnd {
    pub starting_index: usize,
    pub ending_index: usize,
}

*/

#[derive(Event)]
pub enum InputEvent {
    UpEvent,
    DownEvent,
    LeftEvent,
    RightEvent,
    AttackButtonEvent,
}