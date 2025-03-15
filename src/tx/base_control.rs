use crate::tx::protocol::{CommandIds, ToSubPayload};

pub struct BaseControl {
    speed: i16,
    radius: i16,
}

impl BaseControl {
    pub fn new(speed: i16, radius: i16) -> Box<Self> {
        Box::new(BaseControl { speed, radius })
    }
}

impl ToSubPayload for BaseControl {
    fn to_subpayload(&self) -> Vec<u8> {
        vec![
            CommandIds::BaseControl as u8,
            4,
            (self.speed & 0xFF) as u8,
            (self.speed >> 8) as u8,
            (self.radius & 0xFF) as u8,
            (self.radius >> 8) as u8,
        ]
    }
}
