use crate::tx::protocol::{CommandIds, ToSubPayload};
use std::time::Duration;

pub struct Sound {
    note: f32,
    duration: Duration,
}

impl Sound {
    pub fn new(note: f32, duration: Duration) -> Box<Self> {
        Box::new(Self { note, duration })
    }
}

impl ToSubPayload for Sound {
    fn to_subpayload(&self) -> Vec<u8> {
        let note = if self.note == 0.0 {
            0
        } else {
            (1.0 / (self.note * 0.00000275)).round() as u16
        };
        vec![
            CommandIds::Sound as u8,
            3,
            (note & 0xFF) as u8,
            (note >> 8) as u8,
            self.duration.as_millis() as u8,
        ]
    }
}
