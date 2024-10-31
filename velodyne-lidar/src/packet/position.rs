use eyre::{ensure, Result};
use std::mem;

/// Represents a position packet from Velodyne sensor.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PositionPacket {
    pub reserved_head: [u8; 187],
    pub top_board_temperature: u8,
    pub bottom_board_temperature: u8,
    pub last_adc_calibration_temperature: u8,
    pub last_adc_calibration_temperature_change: u16,
    pub seconds_since_last_adc_calibration: u32,
    pub last_adc_calibration_reason: LastAdcCalibrationReason,
    pub adc_calibration_bitmask: u8,
    pub toh: u32,
    pub pps_status: PpsStatus,
    pub thermal_status: ThermalStatus,
    pub last_shutdown_temperature: u8,
    pub temperature_of_unit_at_power_up: u8,
    pub nmea: [u8; 128],
    pub reserved_tail: [u8; 178],
}

impl PositionPacket {
    /// Construct packet from binary buffer.
    pub fn from_bytes(buffer: [u8; mem::size_of::<Self>()]) -> Self {
        unsafe { mem::transmute::<_, Self>(buffer) }
    }

    /// Construct packet from slice of bytes. Fail if the slice size is not correct.
    pub fn from_slice(buffer: &[u8]) -> Result<&Self> {
        ensure!(
            buffer.len() == mem::size_of::<Self>(),
            "Requre the slice length to be {}, but get {}",
            mem::size_of::<Self>(),
            buffer.len(),
        );
        let packet = unsafe { &*(buffer.as_ptr() as *const Self) };
        Ok(packet)
    }

    #[cfg(feature = "nmea")]
    pub fn parse_nmea(&self) -> Result<nmea::ParseResult, nmea::Error<'_>> {
        nmea::parse_bytes(&self.nmea)
    }

    pub fn calibration_in_progress(&self) -> bool {
        self.adc_calibration_bitmask & 0b0001 != 0
    }

    pub fn meet_delta_temperature(&self) -> bool {
        self.adc_calibration_bitmask & 0b0010 != 0
    }

    pub fn meet_periodic_elapsed_time_limit(&self) -> bool {
        self.adc_calibration_bitmask & 0b0100 != 0
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LastAdcCalibrationReason {
    NoCalibration = 0,
    PowerOn = 1,
    Manual = 2,
    DeltaTemperature = 3,
    Periodic = 4,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PpsStatus {
    Abscent = 0,
    Synchronizing = 1,
    Locked = 2,
    Error = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThermalStatus {
    Ok = 0,
    ThermalShutdown = 1,
}
