// game / events.rs

use bevy::{prelude::*, asset::io::embedded};


// not sure
#[derive(Event)]
pub struct  AnimationStart {
    pub starting_index: usize,
    pub ending_index: usize,
    pub is_looping: bool,
}


#[derive(Event)]
pub struct AnimationEnd {
    pub starting_index: usize,
    pub ending_index: usize,
    pub is_looping: bool,
}