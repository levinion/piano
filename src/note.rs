use std::time::Duration;

use rodio::{
    source::{SineWave, TakeDuration},
    Source,
};

#[derive(Clone, Copy, Debug)]
pub enum Pitch {
    C4,
    D4,
    E4,
    F4,
    G4,
    A4,
    B4,
    C5,
    D5,
    E5,
    F5,
    G5,
    A5,
    B5,
    C6,
}

impl From<Pitch> for f32 {
    fn from(value: Pitch) -> Self {
        match value {
            Pitch::C4 => 261.63,
            Pitch::D4 => 293.66,
            Pitch::E4 => 329.63,
            Pitch::F4 => 349.23,
            Pitch::G4 => 392.,
            Pitch::A4 => 440.,
            Pitch::B4 => 493.88,
            Pitch::C5 => 523.25,
            Pitch::D5 => 587.33,
            Pitch::E5 => 659.26,
            Pitch::F5 => 698.46,
            Pitch::G5 => 783.99,
            Pitch::A5 => 880.,
            Pitch::B5 => 987.77,
            Pitch::C6 => 1046.5,
        }
    }
}

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

    pub fn as_wave(&self) -> TakeDuration<SineWave> {
        SineWave::new(self.pitch.into()).take_duration(self.duration)
    }
}
