// game / resources.rs


use bevy::prelude::*;

pub const DEBUGGER_FRAME_ADVANCE_TIMER: f32 = 1.0 / 60.0;

#[derive(Resource)]
pub struct AdvanceOneFrameMode {
    pub should_advance_one_frame: bool,
    pub frame_timer: Timer,
}

impl  Default for AdvanceOneFrameMode {
    fn default() -> AdvanceOneFrameMode {
        AdvanceOneFrameMode {
            should_advance_one_frame: false,
            frame_timer: Timer::from_seconds(DEBUGGER_FRAME_ADVANCE_TIMER, TimerMode::Once),
        }
    }
}