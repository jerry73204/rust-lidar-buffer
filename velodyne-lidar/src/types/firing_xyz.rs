//! Firings in 3D Cartesian coordinates.

use crate::types::{
    format::FormatKind,
    point::{PointD, PointS},
};
use measurements::Angle;
use std::{ops::Range, time::Duration};

macro_rules! declare_firing_xyz {
    ($name:ident, $size:expr, $point:path) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            pub toh: Duration,
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
    use super::{FiringXyzD16, FiringXyzD32, FiringXyzS16, FiringXyzS32};
    use crate::{
        traits::BoxIterator,
        types::{
            format::FormatKind,
            point::{Point, PointRef},
        },
    };
    use std::time::Duration;

    pub type FiringXyz = FormatKind<FiringXyzS16, FiringXyzS32, FiringXyzD16, FiringXyzD32>;

    impl FiringXyz {
        pub fn time(&self) -> Duration {
            match self {
                FiringXyz::Single16(me) => me.toh,
                FiringXyz::Single32(me) => me.toh,
                FiringXyz::Dual16(me) => me.toh,
                FiringXyz::Dual32(me) => me.toh,
            }
        }

        pub fn point_iter(&self) -> BoxIterator<'_, PointRef<'_>> {
            match self {
                FiringXyz::Single16(me) => Box::new(me.points.iter().map(PointRef::from)),
                FiringXyz::Single32(me) => Box::new(me.points.iter().map(PointRef::from)),
                FiringXyz::Dual16(me) => Box::new(me.points.iter().map(PointRef::from)),
                FiringXyz::Dual32(me) => Box::new(me.points.iter().map(PointRef::from)),
            }
        }

        pub fn into_point_iter(self) -> BoxIterator<'static, Point> {
            match self {
                FiringXyz::Single16(me) => Box::new(me.points.into_iter().map(Point::from)),
                FiringXyz::Single32(me) => Box::new(me.points.into_iter().map(Point::from)),
                FiringXyz::Dual16(me) => Box::new(me.points.into_iter().map(Point::from)),
                FiringXyz::Dual32(me) => Box::new(me.points.into_iter().map(Point::from)),
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
    use crate::{traits::BoxIterator, types::point::PointRef};

    pub type FiringXyzRef<'a> =
        FormatKind<&'a FiringXyzS16, &'a FiringXyzS32, &'a FiringXyzD16, &'a FiringXyzD32>;

    impl FiringXyzRef<'_> {
        pub fn time(&self) -> Duration {
            match self {
                FiringXyzRef::Single16(me) => me.toh,
                FiringXyzRef::Single32(me) => me.toh,
                FiringXyzRef::Dual16(me) => me.toh,
                FiringXyzRef::Dual32(me) => me.toh,
            }
        }

        pub fn point_iter(&self) -> BoxIterator<'_, PointRef<'_>> {
            match self {
                FiringXyzRef::Single16(me) => Box::new(me.points.iter().map(PointRef::from)),
                FiringXyzRef::Single32(me) => Box::new(me.points.iter().map(PointRef::from)),
                FiringXyzRef::Dual16(me) => Box::new(me.points.iter().map(PointRef::from)),
                FiringXyzRef::Dual32(me) => Box::new(me.points.iter().map(PointRef::from)),
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
