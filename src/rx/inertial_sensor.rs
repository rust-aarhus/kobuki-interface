#[derive(Clone, Debug, Default, PartialEq)]
pub struct InertialSensor {
    pub angle: f32,
    pub angle_rate: f32,
}

impl TryFrom<&[u8]> for InertialSensor {
    type Error = std::io::Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 7 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid inertial sensor data length: {}", data.len()),
            ));
        }

        Ok(Self {
            angle: i16::from_le_bytes([data[0], data[1]]) as f32 / 100.0,
            angle_rate: i16::from_le_bytes([data[2], data[3]]) as f32 / 100.0,
        })
    }
}
