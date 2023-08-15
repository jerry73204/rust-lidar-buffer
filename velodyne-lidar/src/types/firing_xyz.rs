//! Firings in 3D Cartesian coordinates.

use crate::{
    common::*,
    types::{
        format::FormatKind,
        point::{PointD, PointS},
    },
};

macro_rules! declare_firing_xyz {
    ($name:ident, $size:expr, $point:path) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            pub time: Duration,
            pub azimuth_range: Range<Angle>,
            pub points: [$point; $size],
        }
    };
}

declare_firing_xyz!(FiringXyzS16, 16, PointS);
declare_firing_xyz!(FiringXyzS32, 32, PointS);
declare_firing_xyz!(FiringXyzD16, 16, PointD);
declare_firing_xyz!(FiringXyzD32, 32, PointD);

pub use kind::*;
mod kind {
    use super::*;

    pub type FiringXyz = FormatKind<FiringXyzS16, FiringXyzS32, FiringXyzD16, FiringXyzD32>;

    impl FiringXyz {
        pub fn time(&self) -> Duration {
            match self {
                FiringXyz::Single16(me) => me.time,
                FiringXyz::Single32(me) => me.time,
                FiringXyz::Dual16(me) => me.time,
                FiringXyz::Dual32(me) => me.time,
            }
        }
    }

    impl From<FiringXyzD32> for FiringXyz {
        fn from(v: FiringXyzD32) -> Self {
            Self::Dual32(v)
        }
    }

    impl From<FiringXyzD16> for FiringXyz {
        fn from(v: FiringXyzD16) -> Self {
            Self::Dual16(v)
        }
    }

    impl From<FiringXyzS32> for FiringXyz {
        fn from(v: FiringXyzS32) -> Self {
            Self::Single32(v)
        }
    }

    impl From<FiringXyzS16> for FiringXyz {
        fn from(v: FiringXyzS16) -> Self {
            Self::Single16(v)
        }
    }
}

pub use ref_kind::*;
mod ref_kind {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum FiringXyzRef<'a> {
        Single16(&'a FiringXyzS16),
        Single32(&'a FiringXyzS32),
        Dual16(&'a FiringXyzD16),
        Dual32(&'a FiringXyzD32),
    }

    impl<'a> FiringXyzRef<'a> {
        pub fn time(&self) -> Duration {
            match self {
                FiringXyzRef::Single16(me) => me.time,
                FiringXyzRef::Single32(me) => me.time,
                FiringXyzRef::Dual16(me) => me.time,
                FiringXyzRef::Dual32(me) => me.time,
            }
        }
    }

    impl<'a> From<&'a FiringXyzD32> for FiringXyzRef<'a> {
        fn from(v: &'a FiringXyzD32) -> Self {
            Self::Dual32(v)
        }
    }

    impl<'a> From<&'a FiringXyzD16> for FiringXyzRef<'a> {
        fn from(v: &'a FiringXyzD16) -> Self {
            Self::Dual16(v)
        }
    }

    impl<'a> From<&'a FiringXyzS32> for FiringXyzRef<'a> {
        fn from(v: &'a FiringXyzS32) -> Self {
            Self::Single32(v)
        }
    }

    impl<'a> From<&'a FiringXyzS16> for FiringXyzRef<'a> {
        fn from(v: &'a FiringXyzS16) -> Self {
            Self::Single16(v)
        }
    }
}