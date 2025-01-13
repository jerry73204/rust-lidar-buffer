//! Firings of block references.

use crate::{
    convert::{
        firing_block_to_xyz_d16, firing_block_to_xyz_d32, firing_block_to_xyz_s16,
        firing_block_to_xyz_s32,
    },
    packet::{Block, Channel},
    traits::FiringLike,
    types::{
        channel::ChannelRefD,
        firing_raw::{FiringRawD16, FiringRawD32, FiringRawS16, FiringRawS32},
        firing_xyz::{FiringXyz, FiringXyzD16, FiringXyzD32, FiringXyzS16, FiringXyzS32},
        format::FormatKind,
    },
    Config, Config16, Config32,
};
use eyre::{format_err, Result};
use measurements::Angle;
use std::{ops::Range, time::Duration};

use super::channel_array::{ChannelArrayD, ChannelArrayDRef, ChannelArraySRef};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiringBlockS16<'a> {
    pub toh: Duration,
    pub azimuth_range: Range<Angle>,
    pub block: &'a Block,
    pub channels: ChannelArraySRef<'a, 16>,
}

impl FiringBlockS16<'_> {
    pub fn to_firing_raw(&self) -> FiringRawS16 {
        FiringRawS16 {
            toh: self.toh,
            azimuth_range: self.azimuth_range.clone(),
            channels: *self.channels,
        }
    }

    pub fn to_firing_xyz(&self, beams: &Config16) -> FiringXyzS16 {
        firing_block_to_xyz_s16(self, beams)
    }
}

impl FiringLike for FiringBlockS16<'_> {
    type Point<'p>
        = &'p Channel
    where
        Self: 'p;

    fn start_toh(&self) -> Duration {
        self.toh
    }

    fn num_points(&self) -> usize {
        self.channels.len()
    }

    fn point_at(&self, index: usize) -> Option<Self::Point<'_>> {
        self.channels.get(index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiringBlockS32<'a> {
    pub toh: Duration,
    pub azimuth_range: Range<Angle>,
    pub block: &'a Block,
    pub channels: ChannelArraySRef<'a, 32>,
}

impl FiringBlockS32<'_> {
    pub fn to_firing_raw(&self) -> FiringRawS32 {
        FiringRawS32 {
            toh: self.toh,
            azimuth_range: self.azimuth_range.clone(),
            channels: *self.channels,
        }
    }

    pub fn to_firing_xyz(&self, beams: &Config32) -> FiringXyzS32 {
        firing_block_to_xyz_s32(self, beams)
    }
}

impl FiringLike for FiringBlockS32<'_> {
    type Point<'p>
        = &'p Channel
    where
        Self: 'p;

    fn start_toh(&self) -> Duration {
        self.toh
    }

    fn num_points(&self) -> usize {
        self.channels.len()
    }

    fn point_at(&self, index: usize) -> Option<Self::Point<'_>> {
        self.channels.get(index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiringBlockD16<'a> {
    pub toh: Duration,
    pub azimuth_range: Range<Angle>,
    pub block_strongest: &'a Block,
    pub block_last: &'a Block,
    pub channels: ChannelArrayDRef<'a, 16>,
}

impl<'a> FiringBlockD16<'a> {
    pub fn to_firing_raw(&self) -> FiringRawD16 {
        FiringRawD16 {
            toh: self.toh,
            azimuth_range: self.azimuth_range.clone(),
            channels: ChannelArrayD {
                strongest: *self.channels.strongest,
                last: *self.channels.last,
            },
        }
    }

    pub fn to_firing_xyz(&self, beams: &Config16) -> FiringXyzD16 {
        firing_block_to_xyz_d16(self, beams)
    }

    pub fn strongest_part(&self) -> FiringBlockS16<'a> {
        let Self {
            toh,
            ref azimuth_range,
            block_strongest: block,
            channels:
                ChannelArrayDRef {
                    strongest: channels,
                    ..
                },
            ..
        } = *self;

        FiringBlockS16 {
            toh,
            azimuth_range: azimuth_range.clone(),
            block,
            channels,
        }
    }

    pub fn last_part(&self) -> FiringBlockS16<'a> {
        let Self {
            toh,
            ref azimuth_range,
            block_last: block,
            channels: ChannelArrayDRef { last: channels, .. },
            ..
        } = *self;

        FiringBlockS16 {
            toh,
            azimuth_range: azimuth_range.clone(),
            block,
            channels,
        }
    }
}

impl FiringLike for FiringBlockD16<'_> {
    type Point<'p>
        = ChannelRefD<'p>
    where
        Self: 'p;

    fn start_toh(&self) -> Duration {
        self.toh
    }

    fn num_points(&self) -> usize {
        self.channels.strongest.len()
    }

    fn point_at(&self, index: usize) -> Option<Self::Point<'_>> {
        let strongest = self.channels.strongest.get(index)?;
        let last = self.channels.last.get(index)?;
        Some(ChannelRefD { strongest, last })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiringBlockD32<'a> {
    pub toh: Duration,
    pub azimuth_range: Range<Angle>,
    pub block_strongest: &'a Block,
    pub block_last: &'a Block,
    pub channels: ChannelArrayDRef<'a, 32>,
}

impl<'a> FiringBlockD32<'a> {
    pub fn to_firing_raw(&self) -> FiringRawD32 {
        FiringRawD32 {
            toh: self.toh,
            azimuth_range: self.azimuth_range.clone(),
            channels: ChannelArrayD {
                strongest: *self.channels.strongest,
                last: *self.channels.last,
            },
        }
    }

    pub fn to_firing_xyz(&self, beams: &Config32) -> FiringXyzD32 {
        firing_block_to_xyz_d32(self, beams)
    }

    pub fn strongest_part(&self) -> FiringBlockS32<'a> {
        let Self {
            toh,
            ref azimuth_range,
            block_strongest: block,
            channels:
                ChannelArrayDRef {
                    strongest: channels,
                    ..
                },
            ..
        } = *self;

        FiringBlockS32 {
            toh,
            azimuth_range: azimuth_range.clone(),
            block,
            channels,
        }
    }

    pub fn last_part(&self) -> FiringBlockS32<'a> {
        let Self {
            toh,
            ref azimuth_range,
            block_last: block,
            channels: ChannelArrayDRef { last: channels, .. },
            ..
        } = *self;

        FiringBlockS32 {
            toh,
            azimuth_range: azimuth_range.clone(),
            block,
            channels,
        }
    }
}

impl FiringLike for FiringBlockD32<'_> {
    type Point<'p>
        = ChannelRefD<'p>
    where
        Self: 'p;

    fn start_toh(&self) -> Duration {
        self.toh
    }

    fn num_points(&self) -> usize {
        self.channels.strongest.len()
    }

    fn point_at(&self, index: usize) -> Option<Self::Point<'_>> {
        let strongest = self.channels.strongest.get(index)?;
        let last = self.channels.last.get(index)?;
        Some(ChannelRefD { strongest, last })
    }
}

pub type FiringBlock<'a> =
    FormatKind<FiringBlockS16<'a>, FiringBlockS32<'a>, FiringBlockD16<'a>, FiringBlockD32<'a>>;

impl FiringBlock<'_> {
    pub fn to_firing_xyz(&self, beams: &Config) -> Result<FiringXyz> {
        let err = || format_err!("TODO");

        use FormatKind as F;

        let output = match self {
            F::Single16(inner) => {
                let beams: Config16 = beams.clone().try_into().map_err(|_| err())?;
                inner.to_firing_xyz(&beams).into()
            }
            F::Dual16(inner) => {
                let beams: Config16 = beams.clone().try_into().map_err(|_| err())?;
                inner.to_firing_xyz(&beams).into()
            }
            F::Single32(inner) => {
                let beams: Config32 = beams.clone().try_into().map_err(|_| err())?;
                inner.to_firing_xyz(&beams).into()
            }
            F::Dual32(inner) => {
                let beams: Config32 = beams.clone().try_into().map_err(|_| err())?;
                inner.to_firing_xyz(&beams).into()
            }
        };

        Ok(output)
    }
}

impl<'a> From<FiringBlockD32<'a>> for FiringBlock<'a> {
    fn from(v: FiringBlockD32<'a>) -> Self {
        Self::Dual32(v)
    }
}

impl<'a> From<FiringBlockD16<'a>> for FiringBlock<'a> {
    fn from(v: FiringBlockD16<'a>) -> Self {
        Self::Dual16(v)
    }
}

impl<'a> From<FiringBlockS32<'a>> for FiringBlock<'a> {
    fn from(v: FiringBlockS32<'a>) -> Self {
        Self::Single32(v)
    }
}

impl<'a> From<FiringBlockS16<'a>> for FiringBlock<'a> {
    fn from(v: FiringBlockS16<'a>) -> Self {
        Self::Single16(v)
    }
}
