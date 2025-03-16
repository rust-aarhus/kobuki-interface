use bitflags::bitflags;

#[derive(Clone, Debug, PartialEq)]
pub struct DockingIr {
    pub right: IrSignal,
    pub center: IrSignal,
    pub left: IrSignal,
}

bitflags! {
    /// Represents a set of IR signal flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct IrSignal: u8 {
        /// Near left IR signal detected.
        const NEAR_LEFT = 0x01;
        /// Near center IR signal detected.
        const NEAR_CENTER = 0x02;
        /// Near right IR signal detected.
        const NEAR_RIGHT = 0x04;
        /// Far left IR signal detected.
        const FAR_LEFT = 0x08;
        /// Far center IR signal detected.
        const FAR_CENTER = 0x10;
        /// Far right IR signal detected.
        const FAR_RIGHT = 0x20;
    }
}

impl TryFrom<&[u8]> for DockingIr {
    type Error = std::io::Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 3 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid docking IR data length: {}", data.len()),
            ));
        }

        Ok(DockingIr {
            right: IrSignal::from_bits_truncate(data[0]),
            center: IrSignal::from_bits_truncate(data[1]),
            left: IrSignal::from_bits_truncate(data[2]),
        })
    }
}

impl Default for DockingIr {
    fn default() -> Self {
        Self {
            right: IrSignal::empty(),
            center: IrSignal::empty(),
            left: IrSignal::empty(),
        }
    }
}
