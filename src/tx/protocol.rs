pub struct ByteStream {
    subpayloads: Vec<Box<dyn ToSubPayload>>,
}

pub trait ToSubPayload {
    fn to_subpayload(&self) -> Vec<u8>;
}

impl ByteStream {
    pub fn builder() -> Self {
        ByteStream {
            subpayloads: Vec::new(),
        }
    }

    pub fn subpayload(mut self, subpayload: Box<dyn ToSubPayload>) -> Self {
        self.subpayloads.push(subpayload);
        self
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut payload = Vec::new();
        for subpayload in &self.subpayloads {
            payload.extend(subpayload.to_subpayload());
        }
        let len = payload.len() as u8;

        let mut bytes = Vec::with_capacity(100);
        bytes.push(0xAA);
        bytes.push(0x55);
        bytes.push(len);
        bytes.extend(payload);
        bytes.push(Self::checksum(&bytes));
        bytes
    }

    fn checksum(frame: &[u8]) -> u8 {
        let mut cs = 0;
        for d in frame.iter().skip(2) {
            cs ^= *d;
        }
        cs
    }
}

#[allow(dead_code)]
pub enum CommandIds {
    BaseControl = 1,
    Sound = 3,
    SoundSequence = 4,
    RequestExtra = 9,
    GeneralPurposeOutput = 12,
    SetControllerGain = 13,
    GetControllerGain = 14,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tx::base_control::BaseControl;

    #[test]
    fn test_to_bytes() {
        let payload = ByteStream::builder()
            .subpayload(BaseControl::new(100, -100))
            .to_bytes();
        assert_eq!(
            payload,
            vec![0xAA, 0x55, 0x06, 0x01, 0x04, 0x64, 0x00, 0x9C, 0xFF, 0x04]
        );
    }
}
