// game / events.rs

use bevy::{prelude::*, asset::io::embedded};


// not sure
#[derive(Event)]
pub struct  AnimationStart {
    pub animation_type: String,
    pub starting_index: usize,
    pub ending_index: usize,
}


#[derive(Event)]
pub struct AnimationEnd {
    pub starting_index: usize,
    pub ending_index: usize,
    pub is_looping: bool,
}