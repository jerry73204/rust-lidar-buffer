//! Tools to work with TCP API on Ouster sensors.

use super::{
    consts::PIXELS_PER_COLUMN,
    enums::{LidarMode, MultipurposeIoMode, NmeaBaudRate, OnOffMode, Polarity, TimestampMode},
};
use anyhow::{ensure, format_err, Result};
use derivative::Derivative;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_big_array::big_array;
use std::{
    fmt::{Debug, Display, Error as FormatError, Formatter},
    io::{prelude::*, BufReader, LineWriter, Lines},
    net::{Ipv4Addr, TcpStream, ToSocketAddrs},
    time::Duration,
};

// TODO: This workaround handles large array for serde.
//       We'll remove is it once the const generics is introduced.
big_array! { BigArray; }

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigText {
    pub timestamp_mode: TimestampMode,
    pub multipurpose_io_mode: MultipurposeIoMode,
    pub lidar_mode: LidarMode,
    pub sync_pulse_in_polarity: Polarity,
    pub nmea_in_polarity: Polarity,
    pub sync_pulse_out_polarity: Polarity,
    pub udp_ip: Ipv4Addr,
    #[serde(
        deserialize_with = "de_bool_from_int",
        serialize_with = "ser_bool_to_int"
    )]
    pub nmea_ignore_valid_char: bool,
    #[serde(
        deserialize_with = "de_bool_from_int",
        serialize_with = "ser_bool_to_int"
    )]
    pub auto_start_flag: bool,
    pub sync_pulse_out_pulse_width: u64,
    pub nmea_baud_rate: NmeaBaudRate,
    pub sync_pulse_out_angle: u64,
    pub sync_pulse_out_frequency: u64,
    pub udp_port_imu: u16,
    pub udp_port_lidar: u16,
    pub azimuth_window: [u64; 2],
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct BeamIntrinsics {
    #[serde(with = "BigArray")]
    #[derivative(Debug(format_with = "self::large_array_fmt"))]
    pub beam_altitude_angles: [f64; PIXELS_PER_COLUMN],
    #[serde(with = "BigArray")]
    #[derivative(Debug(format_with = "self::large_array_fmt"))]
    pub beam_azimuth_angles: [f64; PIXELS_PER_COLUMN],
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct LidarIntrinsics {
    pub lidar_to_sensor_transform: [f64; 16],
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct ImuIntrinsics {
    pub imu_to_sensor_transform: [f64; 16],
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct TimeInfo {
    pub timestamp: TimestampInfo,
    pub sync_pulse_in: SyncPulseInInfo,
    pub multipurpose_io: MultiPurposeIo,
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct MultiPurposeIo {
    pub mode: OnOffMode,
    pub sync_pulse_out: SyncPulseOutInfo,
    pub nmea: NmeaInfo,
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct SyncPulseInInfo {
    pub diagnostics: SyncPulseInDiagnosticsInfo,
    pub polarity: Polarity,
    #[serde(
        deserialize_with = "de_bool_from_int",
        serialize_with = "ser_bool_to_int"
    )]
    pub locked: bool,
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct NmeaInfo {
    pub polarity: Polarity,
    pub baud_rate: NmeaBaudRate,
    pub diagnostics: NmeaDiagnosticsInfo,
    pub leap_seconds: u64,
    #[serde(
        deserialize_with = "de_bool_from_int",
        serialize_with = "ser_bool_to_int"
    )]
    pub ignore_valid_char: bool,
    #[serde(
        deserialize_with = "de_bool_from_int",
        serialize_with = "ser_bool_to_int"
    )]
    pub locked: bool,
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct SyncPulseOutInfo {
    pub frequency_hz: u64,
    pub angle_deg: u64,
    pub pulse_width_ms: u64,
    pub polarity: Polarity,
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct TimestampInfo {
    pub time_options: TimeOptionsInfo,
    pub mode: TimestampMode,
    pub time: f64,
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct SyncPulseInDiagnosticsInfo {
    pub count_unfiltered: u64,
    pub last_period_nsec: u64,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct NmeaDiagnosticsInfo {
    pub io_checks: NmeaIoChecksInfo,
    pub decoding: NmeaDecodingInfo,
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct TimeOptionsInfo {
    pub ptp_1588: u64,
    #[serde(
        deserialize_with = "de_bool_from_int",
        serialize_with = "ser_bool_to_int"
    )]
    pub sync_pulse_in: bool,
    pub internal_osc: u64,
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct NmeaIoChecksInfo {
    pub bit_count: u64,
    pub start_char_count: u64,
    pub bit_count_unfilterd: u64,
    pub char_count: u64,
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct NmeaDecodingInfo {
    pub not_valid_count: u64,
    pub last_read_message: String,
    pub utc_decoded_count: u64,
    pub date_decoded_count: u64,
}

pub struct CommandClient {
    reader: Lines<BufReader<TcpStream>>,
    writer: LineWriter<TcpStream>,
}

impl CommandClient {
    pub fn connect<A>(address: A, timeout: Option<Duration>) -> Result<CommandClient>
    where
        A: ToSocketAddrs,
    {
        let stream = TcpStream::connect(&address)?;
        stream.set_read_timeout(timeout)?;
        stream.set_write_timeout(timeout)?;

        let reader = BufReader::new(stream.try_clone()?).lines();
        let writer = LineWriter::new(stream);
        let client = CommandClient { reader, writer };
        Ok(client)
    }

    pub fn get_config_txt(&mut self) -> Result<ConfigText> {
        self.writer.write_all(b"get_config_txt\n")?;
        let line = self
            .reader
            .next()
            .ok_or(format_err!("Unexpected end of stream"))??;
        let config = serde_json::from_str(&line)?;
        Ok(config)
    }

    pub fn get_time_info(&mut self) -> Result<TimeInfo> {
        self.writer.write_all(b"get_time_info\n")?;
        let line = self
            .reader
            .next()
            .ok_or(format_err!("Unexpected end of stream"))??;
        let config = serde_json::from_str(&line)?;
        Ok(config)
    }

    pub fn get_lidar_intrinsics(&mut self) -> Result<LidarIntrinsics> {
        self.writer.write_all(b"get_lidar_intrinsics\n")?;
        let line = self
            .reader
            .next()
            .ok_or(format_err!("Unexpected end of stream"))??;
        let config = serde_json::from_str(&line)?;
        Ok(config)
    }

    pub fn get_imu_intrinsics(&mut self) -> Result<ImuIntrinsics> {
        self.writer.write_all(b"get_imu_intrinsics\n")?;
        let line = self
            .reader
            .next()
            .ok_or(format_err!("Unexpected end of stream"))??;
        let config = serde_json::from_str(&line)?;
        Ok(config)
    }

    pub fn get_beam_intrinsics(&mut self) -> Result<BeamIntrinsics> {
        self.writer.write_all(b"get_beam_intrinsics\n")?;
        let line = self
            .reader
            .next()
            .ok_or(format_err!("Unexpected end of stream"))??;
        let config = serde_json::from_str(&line)?;
        Ok(config)
    }

    pub fn reinitialize(mut self) -> Result<()> {
        self.writer.write_all(b"reinitialize\n")?;
        let line = self
            .reader
            .next()
            .ok_or(format_err!("Unexpected end of stream"))??;
        ensure!(line == "reinitialize", "Unexpected response {:?}", line);
        Ok(())
    }

    pub fn write_config_txt(&mut self) -> Result<()> {
        self.writer.write_all(b"write_config_txt\n")?;
        let line = self
            .reader
            .next()
            .ok_or(format_err!("Unexpected end of stream"))??;
        ensure!(line == "write_config_txt", "Unexpected response {:?}", line);
        Ok(())
    }

    pub fn set_udp_ip(&mut self, ip: Ipv4Addr) -> Result<()> {
        self.set_config_param("udp_ip", ip)?;
        Ok(())
    }

    pub fn set_udp_port_lidar(&mut self, port: u16) -> Result<()> {
        self.set_config_param("udp_port_lidar", port)?;
        Ok(())
    }

    pub fn set_udp_port_imu(&mut self, port: u16) -> Result<()> {
        self.set_config_param("udp_port_imu", port)?;
        Ok(())
    }

    pub fn set_lidar_mode(&mut self, mode: LidarMode) -> Result<()> {
        self.set_config_param("lidar_mode", mode)?;
        Ok(())
    }

    pub fn set_timestamp_mode(&mut self, mode: TimestampMode) -> Result<()> {
        self.set_config_param("timestamp_mode", mode)?;
        Ok(())
    }

    pub fn set_sync_pulse_in_polarity(&mut self, polarity: Polarity) -> Result<()> {
        self.set_config_param("sync_pulse_in_polarity", polarity)?;
        Ok(())
    }

    pub fn set_nmea_in_polarity(&mut self, polarity: Polarity) -> Result<()> {
        self.set_config_param("nmea_in_polarity", polarity)?;
        Ok(())
    }

    fn set_config_param<T: Display>(&mut self, param: &str, arg: T) -> Result<()> {
        let command = format!("set_config_param {} {}\n", param, arg);
        self.writer.write_all(command.as_bytes())?;
        let line = self
            .reader
            .next()
            .ok_or(format_err!("Unexpected end of stream"))??;
        ensure!(line == "set_config_param", "Unexpected response {:?}", line);
        Ok(())
    }
}

fn ser_bool_to_int<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        true => serializer.serialize_u64(1),
        false => serializer.serialize_u64(0),
    }
}

fn de_bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(deserializer)? {
        1 => Ok(true),
        0 => Ok(false),
        other => {
            use serde::de::{Error, Unexpected};
            let error = Error::invalid_value(Unexpected::Unsigned(other), &"Expect 0 or 1");
            Err(error)
        }
    }
}

fn large_array_fmt<T: Debug>(
    array: &[T; PIXELS_PER_COLUMN],
    formatter: &mut Formatter,
) -> Result<(), FormatError> {
    write!(formatter, "{:?}", array as &[_])
}
