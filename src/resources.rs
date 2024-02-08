// main resources

use::bevy::prelude::*;

// I don't need to have a custom implementation for Default b/c I am deriving Default here
// I think I'd need a custom impl for it if the struct had its own parameters with defined types?
#[derive(Resource, Default)]
pub struct MouseCursorWorldCoordinates(pub Vec2);