use std::time::Duration;

use midia::pitch::Pitch;

#[derive(Clone, Copy, Debug)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: Duration,
}

impl Note {
    pub fn new(pitch: Pitch, duration: f32) -> Self {
        Self {
            pitch,
            duration: Duration::from_secs_f32(duration),
        }
    }
}
