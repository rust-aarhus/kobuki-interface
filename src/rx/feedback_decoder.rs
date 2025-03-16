/// Data format:
///   header0: 0xaa
///   header1: 0x55
///   length: Length of payload
///   payload
///   checksum: Checksum of length and payload
///
/// Example data:
/// [aa, 55, 4d, 01, 0f, a8, 0b, 00, 00, 00, fc, ff, ef, f3, 00, 00, 00, 00, 9d, 00, 03, 03, 00, 00, 00, 04, 07, 9b, e7, 00, 00, 00, 00, 00, 05, 06, 89, 06, ad, 07, 49, 06, 06, 02, 00, 00, 0d, 0e, ee, 06, 2a, 00, 92, ff, bc, ff, 23, 00, 98, ff, b9, ff, 10, 10, 00, 00, ff, 0f, ff, 0f, ff, 0f, ff, 0f, f0, 0f, 00, 00, 00, 00, 62]
///  ------  --  --------------------------------------  --------------------------------------  --------------------------------------  --------------------------------------  --------------------------------------  --------------------------------------  --------------------------------------  --------------------------  --
///  header  len payload (10 bytes)                      payload (10 bytes)                      payload (10 bytes)                      payload (10 bytes)                      payload (10 bytes)                      payload (10 bytes)                      payload (10 bytes)                      payload (7 bytes)           chk
///          77
///              --  --  ----------------------------------------------------------  --  --  ----------
///              id  len payload                                                     id  len payload
///
use super::feedback::Feedback;
use bytes::{Buf, BytesMut};
use tokio_util::codec::Decoder;

pub struct FeedbackDecoder;

impl Decoder for FeedbackDecoder {
    type Item = Feedback;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // Find the header
        if let Some((header_index, _)) =
            src.windows(2).enumerate().find(|(_, n)| *n == [0xaa, 0x55])
        {
            src.advance(header_index);
        } else {
            src.clear();
            return Ok(None);
        }

        if src.len() < 3 {
            // Not enough data to read length marker.
            return Ok(None);
        }

        // Read length
        let length = src[2] as usize;

        // Check we have a full frame (length + 2 header bytes + 1 length byte + 1 checksum byte)
        if src.len() < 4 + length {
            return Ok(None);
        }

        // Check the checksum
        let checksum_index = 3 + length;
        if checksum_index >= src.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Checksum index out of bounds",
            ));
        }
        let checksum = src[checksum_index];
        let calculated_checksum = src[2..3 + length].iter().fold(0u8, |acc, x| acc ^ *x);
        if checksum != calculated_checksum {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Checksum mismatch. got {}, expected {}",
                    checksum, calculated_checksum
                ),
            ));
        }

        // Handle the sub-payloads
        let mut feedback = Feedback::new();
        let mut i = 3;
        while i < checksum_index {
            let id = FeedbackId::try_from(src[i])?;
            let sub_length = src[i + 1] as usize;
            let next_i = i + 2 + sub_length;
            if next_i > checksum_index {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Sub-payload extends past checksum",
                ));
            }
            feedback.add_subpayload(id, src[i + 2..next_i].as_ref())?;
            i = next_i;
        }

        src.advance(3 + length + 1);

        Ok(Some(feedback))
    }
}

pub enum FeedbackId {
    BasicSensorData = 1,
    DockingIR = 3,
    InertialSensor = 4,
    Cliff = 5,
    Current = 6,
    HardwareVersion = 10,
    FirmwareVersion = 11,
    Gyro = 13,
    GeneralPurposeInput = 16,
    UniqueDeviceId = 19,
    ControllerInfo = 21,
}

impl TryFrom<u8> for FeedbackId {
    type Error = std::io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FeedbackId::BasicSensorData),
            3 => Ok(FeedbackId::DockingIR),
            4 => Ok(FeedbackId::InertialSensor),
            5 => Ok(FeedbackId::Cliff),
            6 => Ok(FeedbackId::Current),
            10 => Ok(FeedbackId::HardwareVersion),
            11 => Ok(FeedbackId::FirmwareVersion),
            13 => Ok(FeedbackId::Gyro),
            16 => Ok(FeedbackId::GeneralPurposeInput),
            19 => Ok(FeedbackId::UniqueDeviceId),
            21 => Ok(FeedbackId::ControllerInfo),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown feedback id {}", value),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rx::docking_ir::{DockingIr, IrSignal};

    fn create_docking_ir(right: IrSignal) -> (BytesMut, Feedback) {
        // [aa, 55, 05, 03, 03, xx, 00, 00, 00]
        //  ------  --  --  --  ----------  --
        //  header  len id  len payload     chk
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&[0xaa, 0x55, 0x05, 0x03, 0x03, 0x00, 0x00, 0x00, 0x00]);
        bytes[5] = right.bits();
        let docking_ir = DockingIr {
            right,
            center: IrSignal::empty(),
            left: IrSignal::empty(),
        };
        let checksum = bytes[2..8].iter().fold(0u8, |acc, x| acc ^ *x);
        bytes[8] = checksum;

        let mut feedback = Feedback::new();
        feedback.docking_ir = Some(docking_ir);
        (bytes, feedback)
    }

    #[test]
    fn test_no_header() {
        let mut decoder = FeedbackDecoder;
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&[0x00, 0x00, 0x00]);
        let result = decoder.decode(&mut buf).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_full_frame() {
        let (mut bytes, expected) = create_docking_ir(IrSignal::NEAR_LEFT);
        let mut decoder = FeedbackDecoder;
        let result = decoder.decode(&mut bytes).unwrap();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_full_frame_garbage_before_frame() {
        let (frame_bytes, expected) = create_docking_ir(IrSignal::NEAR_LEFT);
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&[0x00, 0x55, 0x00]);
        bytes.extend_from_slice(&frame_bytes);
        let mut decoder = FeedbackDecoder;
        let result = decoder.decode(&mut bytes).unwrap();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_full_frame_garbage_after_frame() {
        let (mut bytes, expected) = create_docking_ir(IrSignal::NEAR_LEFT);
        bytes.extend_from_slice(&[0x00, 0x55, 0x00]);
        let mut decoder = FeedbackDecoder;
        let result = decoder.decode(&mut bytes).unwrap();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_two_half_frames() {
        let (mut bytes, expected) = create_docking_ir(IrSignal::NEAR_LEFT);
        let second_half = bytes.split_off(5);
        let mut decoder = FeedbackDecoder;
        let result = decoder.decode(&mut bytes).unwrap();
        assert_eq!(result, None);

        bytes.extend_from_slice(&second_half);
        let result = decoder.decode(&mut bytes).unwrap();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_two_full_frames() {
        let (bytes_first, expected_first) = create_docking_ir(IrSignal::NEAR_LEFT);
        let (bytes_second, expected_second) = create_docking_ir(IrSignal::NEAR_RIGHT);
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&bytes_first);
        bytes.extend_from_slice(&bytes_second);
        let mut decoder = FeedbackDecoder;
        let result = decoder.decode(&mut bytes).unwrap();
        assert_eq!(result, Some(expected_first));
        let result = decoder.decode(&mut bytes).unwrap();
        assert_eq!(result, Some(expected_second));
        let result = decoder.decode(&mut bytes).unwrap();
        assert_eq!(result, None);
    }
}
