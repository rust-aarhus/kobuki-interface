use crate::tx::protocol::{CommandIds, ToSubPayload};
use std::time::Duration;

pub struct Sound {
    note: u16,
    duration: Duration,
}

impl Sound {
    pub fn new(note: u16, duration: Duration) -> Box<Self> {
        Box::new(Self { note, duration })
    }
}

impl ToSubPayload for Sound {
    fn to_subpayload(&self) -> Vec<u8> {
        vec![
            CommandIds::Sound as u8,
            3,
            (self.note & 0xFF) as u8,
            (self.note >> 8) as u8,
            self.duration.as_millis() as u8,
        ]
    }
}
