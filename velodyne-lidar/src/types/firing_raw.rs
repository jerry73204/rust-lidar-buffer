//! Firings of blocks.

use crate::types::format::FormatKind;
use measurements::Angle;
use std::{ops::Range, time::Duration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiringRawS16 {
    pub toh: Duration,
    pub azimuth_range: Range<Angle>,
    pub channels: ChannelArrayS<16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiringRawS32 {
    pub toh: Duration,
    pub azimuth_range: Range<Angle>,
    pub channels: ChannelArrayS<32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiringRawD16 {
    pub toh: Duration,
    pub azimuth_range: Range<Angle>,
    pub channels: ChannelArrayD<16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiringRawD32 {
    pub toh: Duration,
    pub azimuth_range: Range<Angle>,
    pub channels: ChannelArrayD<32>,
}

pub use kind::*;
mod kind {
    use super::*;

    pub type FiringRaw = FormatKind<FiringRawS16, FiringRawS32, FiringRawD16, FiringRawD32>;

    impl FiringRaw {
        pub fn time(&self) -> Duration {
            match self {
                FiringRaw::Single16(me) => me.toh,
                FiringRaw::Single32(me) => me.toh,
                FiringRaw::Dual16(me) => me.toh,
                FiringRaw::Dual32(me) => me.toh,
            }
        }
    }

    impl From<FiringRawD32> for FiringRaw {
        fn from(v: FiringRawD32) -> Self {
            Self::Dual32(v)
        }
    }

    impl From<FiringRawD16> for FiringRaw {
        fn from(v: FiringRawD16) -> Self {
            Self::Dual16(v)
        }
    }

    impl From<FiringRawS32> for FiringRaw {
        fn from(v: FiringRawS32) -> Self {
            Self::Single32(v)
        }
    }

    impl From<FiringRawS16> for FiringRaw {
        fn from(v: FiringRawS16) -> Self {
            Self::Single16(v)
        }
    }
}

pub use ref_kind::*;

use super::channel_array::{ChannelArrayD, ChannelArrayS};
mod ref_kind {
    use super::*;

    pub type FiringRawRef<'a> =
        FormatKind<&'a FiringRawS16, &'a FiringRawS32, &'a FiringRawD16, &'a FiringRawD32>;

    impl FiringRawRef<'_> {
        pub fn time(&self) -> Duration {
            match self {
                FiringRawRef::Single16(me) => me.toh,
                FiringRawRef::Single32(me) => me.toh,
                FiringRawRef::Dual16(me) => me.toh,
                FiringRawRef::Dual32(me) => me.toh,
            }
        }
    }

    impl<'a> From<&'a FiringRawD32> for FiringRawRef<'a> {
        fn from(v: &'a FiringRawD32) -> Self {
            Self::Dual32(v)
        }
    }

    impl<'a> From<&'a FiringRawD16> for FiringRawRef<'a> {
        fn from(v: &'a FiringRawD16) -> Self {
            Self::Dual16(v)
        }
    }

    impl<'a> From<&'a FiringRawS32> for FiringRawRef<'a> {
        fn from(v: &'a FiringRawS32) -> Self {
            Self::Single32(v)
        }
    }

    impl<'a> From<&'a FiringRawS16> for FiringRawRef<'a> {
        fn from(v: &'a FiringRawS16) -> Self {
            Self::Single16(v)
        }
    }
}
