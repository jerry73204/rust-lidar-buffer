use super::{DataPacket, PositionPacket};
use eyre::{bail, Result};

/// An enumeration of data packet and position packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Data(Box<DataPacket>),
    Position(Box<PositionPacket>),
}

impl Packet {
    pub fn from_slice(buffer: &[u8]) -> Result<Self> {
        Ok(if let Ok(packet) = DataPacket::from_slice(buffer) {
            (*packet).into()
        } else if let Ok(packet) = PositionPacket::from_slice(buffer) {
            (*packet).into()
        } else {
            bail!("unable to parse bytes into a data or a positoin packet");
        })
    }

    pub fn try_into_data(self) -> Result<DataPacket, Self> {
        if let Self::Data(v) = self {
            Ok(*v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_position(self) -> Result<PositionPacket, Self> {
        if let Self::Position(v) = self {
            Ok(*v)
        } else {
            Err(self)
        }
    }

    pub fn as_data(&self) -> Option<&DataPacket> {
        if let Self::Data(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_position(&self) -> Option<&PositionPacket> {
        if let Self::Position(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<PositionPacket> for Packet {
    fn from(v: PositionPacket) -> Self {
        Self::Position(Box::new(v))
    }
}

impl From<DataPacket> for Packet {
    fn from(v: DataPacket) -> Self {
        Self::Data(Box::new(v))
    }
}
