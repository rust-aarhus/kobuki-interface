use bitflags::bitflags;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BasicSensorData {
    pub timestamp: u16,
    pub bumper: SidesCentral,
    pub wheel_drop: Sides,
    pub cliff: SidesCentral,
    pub left_encoder: u16,
    pub right_encoder: u16,
    pub left_pwm: i8,
    pub right_pwm: i8,
    pub button: Button,
    pub charger: Charger,
    pub battery: f32,
    pub overcurrent_wheel: Sides,
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SidesCentral: u8 {
        const RIGHT = 0x01;
        const CENTRAL = 0x02;
        const LEFT = 0x04;
    }
}

impl std::fmt::Display for SidesCentral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sides = Vec::new();
        if self.contains(Self::RIGHT) {
            sides.push("right");
        }
        if self.contains(Self::CENTRAL) {
            sides.push("central");
        }
        if self.contains(Self::LEFT) {
            sides.push("left");
        }
        write!(f, "{}", sides.join(", "))
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Sides: u8 {
        const RIGHT = 0x01;
        const LEFT = 0x02;
    }
}

impl std::fmt::Display for Sides {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sides = Vec::new();
        if self.contains(Self::RIGHT) {
            sides.push("right");
        }
        if self.contains(Self::LEFT) {
            sides.push("left");
        }
        write!(f, "{}", sides.join(" and "))
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Button: u8 {
        const B0 = 0x01;
        const B1 = 0x02;
        const B2 = 0x04;
    }
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sides = Vec::new();
        if self.contains(Self::B0) {
            sides.push("Button 0");
        }
        if self.contains(Self::B1) {
            sides.push("Button 1");
        }
        if self.contains(Self::B2) {
            sides.push("Button 2");
        }
        write!(f, "{}", sides.join(", "))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Charger {
    #[default]
    Discharging = 0,
    DockingCharged = 2,
    DockingCharging = 6,
    AdapterCharged = 18,
    AdapterCharging = 22,
}

impl TryFrom<u8> for Charger {
    type Error = std::io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Discharging),
            2 => Ok(Self::DockingCharged),
            6 => Ok(Self::DockingCharging),
            18 => Ok(Self::AdapterCharged),
            22 => Ok(Self::AdapterCharging),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid charger value: {}", value),
            )),
        }
    }
}

impl TryFrom<&[u8]> for BasicSensorData {
    type Error = std::io::Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 15 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid inertial sensor data length: {}", data.len()),
            ));
        }

        Ok(Self {
            timestamp: u16::from_le_bytes([data[0], data[1]]),
            bumper: SidesCentral::from_bits_truncate(data[2]),
            wheel_drop: Sides::from_bits_truncate(data[3]),
            cliff: SidesCentral::from_bits_truncate(data[4]),
            left_encoder: u16::from_le_bytes([data[5], data[6]]),
            right_encoder: u16::from_le_bytes([data[7], data[8]]),
            left_pwm: data[9] as i8,
            right_pwm: data[10] as i8,
            button: Button::from_bits_truncate(data[11]),
            charger: Charger::try_from(data[12])?,
            battery: data[13] as f32 / 10.0,
            overcurrent_wheel: Sides::from_bits_truncate(data[14]),
        })
    }
}
