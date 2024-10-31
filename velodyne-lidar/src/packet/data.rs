use crate::{
    consts::{AZIMUTH_COUNT_PER_REV, BLOCKS_PER_PACKET, CHANNELS_PER_BLOCK, FIRING_PERIOD},
    types::{
        channel_array::ChannelArrayDRef,
        firing_block::{FiringBlockD16, FiringBlockD32, FiringBlockS16, FiringBlockS32},
        firing_xyz::{FiringXyzD16, FiringXyzD32, FiringXyzS16, FiringXyzS32},
        format::{Format, FormatKind},
    },
    utils::AngleExt as _,
    Config16, Config32,
};
use eyre::{ensure, Result};
use itertools::{chain, izip, Itertools as _};
use measurements::Angle;
use std::{f64::consts::PI, iter, mem, time::Duration};

/// Represents the block index in range from 0 to 31, or from 32 to 63.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockIdentifier {
    Block0To31 = 0xeeff,
    Block32To63 = 0xddff,
}

/// Represents the way the sensor measures the laser signal.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReturnMode {
    Strongest = 0x37,
    Last = 0x38,
    Dual = 0x39,
}

impl ReturnMode {
    pub fn is_single(&self) -> bool {
        [ReturnMode::Strongest, ReturnMode::Last].contains(self)
    }

    pub fn is_dual(&self) -> bool {
        *self == Self::Dual
    }
}

/// Represents the hardware model.
#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, strum::AsRefStr, strum::Display, strum::EnumString,
)]
pub enum ProductID {
    HDL32E = 0x21,
    VLP16 = 0x22,
    PuckLite = 0x23,
    PuckHiRes = 0x24,
    VLP32C = 0x28,
    Velarray = 0x31,
    VLS128 = 0xa1,
}

impl ProductID {
    pub fn num_lines(&self) -> usize {
        match self {
            Self::HDL32E => 16,
            Self::VLP16 => 16,
            Self::PuckLite => 16,
            Self::PuckHiRes => 16,
            Self::VLP32C => 32,
            Self::Velarray => todo!(),
            Self::VLS128 => 128,
        }
    }
}

/// Represents a point of measurement.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Channel {
    /// The raw distance of laser return.
    pub distance: u16,
    /// The intensity of laser return.
    pub intensity: u8,
}

/// Represents a sequence of measurements with meta data.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block {
    /// Represents the block that the firing belongs to.
    pub block_identifier: BlockIdentifier,
    /// Encoder count of rotation motor ranging from 0 to 36000 (inclusive).
    pub azimuth_count: u16,
    /// Array of channels.
    pub channels: [Channel; CHANNELS_PER_BLOCK],
}

impl Block {
    pub fn azimuth_radians(&self) -> f64 {
        2.0 * PI * self.azimuth_count as f64 / (AZIMUTH_COUNT_PER_REV - 1) as f64
    }

    pub fn azimuth_degrees(&self) -> f64 {
        360.0 * self.azimuth_count as f64 / (AZIMUTH_COUNT_PER_REV - 1) as f64
    }

    pub fn azimuth(&self) -> Angle {
        Angle::from_radians(self.azimuth_radians())
    }
}

/// Represents a data packet from Velodyne sensor.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataPacket {
    /// Sensor data.
    pub blocks: [Block; BLOCKS_PER_PACKET],
    /// Timestamp in microseconds.
    pub toh: u32,
    /// Indicates single return mode or dual return mode.
    pub return_mode: ReturnMode,
    /// Sensor model.
    pub product_id: ProductID,
}

impl DataPacket {
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

    pub fn toh(&self) -> Duration {
        Duration::from_micros(self.toh as u64)
    }

    pub fn try_format(&self) -> Option<Format> {
        Format::try_from_model(self.product_id, self.return_mode)
    }

    pub fn format(&self) -> Format {
        Format::from_model(self.product_id, self.return_mode)
    }

    pub fn firing_block_iter(
        &self,
    ) -> FormatKind<
        impl Iterator<Item = FiringBlockS16<'_>> + Clone,
        impl Iterator<Item = FiringBlockS32<'_>> + Clone,
        impl Iterator<Item = FiringBlockD16<'_>> + Clone,
        impl Iterator<Item = FiringBlockD32<'_>> + Clone,
    > {
        use Format::*;
        use FormatKind as F;

        match self.format() {
            Single16 => F::from_s16(self.firing_block_iter_s16()),
            Dual16 => F::from_d16(self.firing_block_iter_d16()),
            Single32 => F::from_s32(self.firing_block_iter_s32()),
            Dual32 => F::from_d32(self.firing_block_iter_d32()),
        }
    }

    pub fn firing_block_iter_s16(
        &self,
    ) -> impl Iterator<Item = FiringBlockS16<'_>> + Clone + Sync + Send {
        let block_period = FIRING_PERIOD.mul_f64(2.0);
        let tohs = iter::successors(Some(self.toh()), move |prev| Some(*prev + block_period));
        let firing_azimuths = {
            let block_azimuths: Vec<_> = self.blocks.iter().map(|block| block.azimuth()).collect();
            let block_azimuth_diffs: Vec<_> = block_azimuths
                .iter()
                .cloned()
                .tuple_windows()
                .map(|(curr, next)| (next - curr).wrap_to_2pi())
                .collect();
            let last_block_azimuth_diff = *block_azimuth_diffs.last().unwrap();

            izip!(
                block_azimuths,
                chain!(block_azimuth_diffs, [last_block_azimuth_diff])
            )
            .map(|(block_azimuth, block_azimuth_diff)| {
                let mid_azimuth = block_azimuth + block_azimuth_diff / 2.0;
                let last_azimuth = block_azimuth + block_azimuth_diff;
                [block_azimuth..mid_azimuth, mid_azimuth..last_azimuth]
            })
        };

        izip!(tohs, firing_azimuths, &self.blocks).flat_map(
            move |(block_toh, [former_azimuth, latter_azimuth], block)| {
                let former_toh = block_toh;
                let latter_toh = former_toh + FIRING_PERIOD;

                let (former_channels, latter_channels) = block.channels.split_at(16);

                let former = FiringBlockS16 {
                    toh: former_toh,
                    azimuth_range: former_azimuth,
                    block,
                    channels: former_channels
                        .try_into()
                        .unwrap_or_else(|_| unreachable!()),
                };
                let latter = FiringBlockS16 {
                    toh: latter_toh,
                    azimuth_range: latter_azimuth,
                    block,
                    channels: latter_channels
                        .try_into()
                        .unwrap_or_else(|_| unreachable!()),
                };

                [former, latter]
            },
        )
    }

    pub fn firing_block_iter_d16(
        &self,
    ) -> impl Iterator<Item = FiringBlockD16<'_>> + Clone + Sync + Send {
        let block_period = FIRING_PERIOD.mul_f64(2.0);
        let tohs = iter::successors(Some(self.toh()), move |prev| Some(*prev + block_period));
        let firing_azimuths = {
            let block_azimuths: Vec<_> = self
                .blocks
                .iter()
                .step_by(2)
                .map(|block| block.azimuth())
                .collect();
            let block_azimuth_diffs: Vec<_> = block_azimuths
                .iter()
                .cloned()
                .tuple_windows()
                .map(|(curr, next)| (next - curr).wrap_to_2pi())
                .collect();
            let last_block_azimuth_diff = *block_azimuth_diffs.last().unwrap();

            izip!(
                block_azimuths,
                chain!(block_azimuth_diffs, [last_block_azimuth_diff])
            )
            .map(|(block_azimuth, block_azimuth_diff)| {
                let mid_azimuth = block_azimuth + block_azimuth_diff / 2.0;
                let last_azimuth = block_azimuth + block_azimuth_diff;
                [block_azimuth..mid_azimuth, mid_azimuth..last_azimuth]
            })
        };

        izip!(tohs, firing_azimuths, self.blocks.chunks(2)).flat_map(
            |(block_toh, [former_azimuth, latter_azimuth], block_pair)| {
                let [block_strongest, block_last] = match block_pair {
                    [first, second] => [first, second],
                    _ => unreachable!(),
                };

                let former_toh = block_toh;
                let latter_toh = former_toh + FIRING_PERIOD;

                let (former_strongest, latter_strongest) = block_strongest.channels.split_at(16);
                let (former_last, latter_last) = block_last.channels.split_at(16);

                [
                    FiringBlockD16 {
                        toh: former_toh,
                        azimuth_range: former_azimuth,
                        block_strongest,
                        block_last,
                        channels: ChannelArrayDRef {
                            strongest: former_strongest
                                .try_into()
                                .unwrap_or_else(|_| unreachable!()),
                            last: former_last.try_into().unwrap_or_else(|_| unreachable!()),
                        },
                    },
                    FiringBlockD16 {
                        toh: latter_toh,
                        azimuth_range: latter_azimuth,
                        block_strongest,
                        block_last,
                        channels: ChannelArrayDRef {
                            strongest: latter_strongest
                                .try_into()
                                .unwrap_or_else(|_| unreachable!()),
                            last: latter_last.try_into().unwrap_or_else(|_| unreachable!()),
                        },
                    },
                ]
            },
        )
    }

    pub fn firing_block_iter_s32(
        &self,
    ) -> impl Iterator<Item = FiringBlockS32<'_>> + Clone + Sync + Send {
        let tohs = iter::successors(Some(self.toh()), move |prev| Some(*prev + FIRING_PERIOD));
        let azimuths = {
            let block_azimuths: Vec<_> = self.blocks.iter().map(|block| block.azimuth()).collect();
            let block_azimuth_diffs: Vec<_> = block_azimuths
                .iter()
                .cloned()
                .tuple_windows()
                .map(|(curr, next)| (next - curr).wrap_to_2pi())
                .collect();
            let last_block_azimuth_diff = *block_azimuth_diffs.last().unwrap();

            izip!(
                block_azimuths,
                chain!(block_azimuth_diffs, [last_block_azimuth_diff])
            )
            .map(|(former_azimuth, azimuth_diff)| {
                let latter_azimuth = former_azimuth + azimuth_diff;
                former_azimuth..latter_azimuth
            })
        };

        izip!(tohs, azimuths, &self.blocks).map(move |(block_toh, azimuth_range, block)| {
            let former_toh = block_toh;
            let latter_toh = former_toh + FIRING_PERIOD;

            FiringBlockS32 {
                toh: latter_toh,
                azimuth_range,
                block,
                channels: &block.channels,
            }
        })
    }

    pub fn firing_block_iter_d32(
        &self,
    ) -> impl Iterator<Item = FiringBlockD32<'_>> + Clone + Sync + Send {
        let tohs = iter::successors(Some(self.toh()), move |prev| Some(*prev + FIRING_PERIOD));
        let azimuths = {
            let azimuths: Vec<_> = self
                .blocks
                .iter()
                .step_by(2)
                .map(|block| block.azimuth())
                .collect();
            let azimuth_diffs: Vec<_> = azimuths
                .iter()
                .cloned()
                .tuple_windows()
                .map(|(curr, next)| (next - curr).wrap_to_2pi())
                .collect();
            let last_azimuth_diff = *azimuth_diffs.last().unwrap();

            izip!(azimuths, chain!(azimuth_diffs, [last_azimuth_diff])).map(
                |(former_azimuth, azimuth_diff)| {
                    let latter_azimuth = former_azimuth + azimuth_diff;
                    former_azimuth..latter_azimuth
                },
            )
        };

        izip!(tohs, azimuths, self.blocks.chunks(2)).map(
            move |(block_toh, azimuth_range, chunk)| {
                let [block_strongest, block_last] = match chunk {
                    [first, second] => [first, second],
                    _ => unreachable!(),
                };

                FiringBlockD32 {
                    toh: block_toh,
                    azimuth_range,
                    block_strongest,
                    block_last,
                    channels: ChannelArrayDRef {
                        strongest: &block_strongest.channels,
                        last: &block_last.channels,
                    },
                }
            },
        )
    }

    pub fn firing_xyz_iter_s16<'a>(
        &'a self,
        beams: &'a Config16,
    ) -> impl Iterator<Item = FiringXyzS16> + Clone + Sync + Send + 'a {
        self.firing_block_iter_s16()
            .map(|firing| firing.to_firing_xyz(beams))
    }

    pub fn firing_xyz_iter_s32<'a>(
        &'a self,
        beams: &'a Config32,
    ) -> impl Iterator<Item = FiringXyzS32> + Clone + Sync + Send + 'a {
        self.firing_block_iter_s32()
            .map(|firing| firing.to_firing_xyz(beams))
    }

    pub fn firing_xyz_iter_d16<'a>(
        &'a self,
        beams: &'a Config16,
    ) -> impl Iterator<Item = FiringXyzD16> + Clone + Sync + Send + 'a {
        self.firing_block_iter_d16()
            .map(|firing| firing.to_firing_xyz(beams))
    }

    pub fn firing_xyz_iter_d32<'a>(
        &'a self,
        beams: &'a Config32,
    ) -> impl Iterator<Item = FiringXyzD32> + Clone + Sync + Send + 'a {
        self.firing_block_iter_d32()
            .map(|firing| firing.to_firing_xyz(beams))
    }
}
