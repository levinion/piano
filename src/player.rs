use std::{thread::sleep, time::Duration};

use rodio::Sink;

use crate::note::{Note, Pitch};

pub struct Player {
    speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self { speed: 1. }
    }

    pub fn set_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn play(&self, data: &str) {
        let notes = Self::parse_notes(data, self.speed);
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        notes.iter().for_each(|note| {
            let mut wave = note.as_wave();
            wave.set_filter_fadeout();
            sink.append(wave);
        });
        sink.sleep_until_end();
        sleep(Duration::from_secs_f32(0.1));
    }

    fn parse_notes(data: &str, speed: f32) -> Vec<Note> {
        data.split_whitespace()
            .flat_map(|note| {
                match note {
                    "|" | "\n" => return None,
                    _ => {}
                }
                let mut duration = 1. / speed;
                let note: Vec<_> = note.split('/').collect();
                if note.len() > 1 {
                    duration *= note[1].parse::<f32>().unwrap();
                }
                let note = match note[0] {
                    "c4" => Note::new(Pitch::C4, duration),
                    "d4" => Note::new(Pitch::D4, duration),
                    "e4" => Note::new(Pitch::E4, duration),
                    "f4" => Note::new(Pitch::F4, duration),
                    "g4" => Note::new(Pitch::G4, duration),
                    "a4" => Note::new(Pitch::A4, duration),
                    "b4" => Note::new(Pitch::B4, duration),
                    "c5" => Note::new(Pitch::C5, duration),
                    "d5" => Note::new(Pitch::D5, duration),
                    "e5" => Note::new(Pitch::E5, duration),
                    "f5" => Note::new(Pitch::F5, duration),
                    "g5" => Note::new(Pitch::G5, duration),
                    "a5" => Note::new(Pitch::A5, duration),
                    "b5" => Note::new(Pitch::B5, duration),
                    "c6" => Note::new(Pitch::C6, duration),
                    _ => unreachable!(),
                };
                Some(note)
            })
            .collect()
    }
}
