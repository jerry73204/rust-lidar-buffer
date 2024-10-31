//! Defines a set of Velodyne LiDAR configurations.

use crate::{consts, packet::ReturnMode};
use eyre::Result;
use itertools::izip;
use measurements::{Angle, Length};

pub use config_::*;
mod config_ {
    use crate::types::format::{Format, FormatKind};

    use super::*;

    // type

    /// Config type for Velodyne LiDARs.
    #[derive(Debug, Clone)]
    pub struct Config {
        pub return_mode: ReturnMode,
        pub distance_resolution: Length,
        pub lasers: Vec<Beam>,
    }

    #[derive(Debug, Clone)]
    pub struct Config16 {
        pub return_mode: ReturnMode,
        pub distance_resolution: Length,
        pub lasers: [Beam; 16],
    }

    #[derive(Debug, Clone)]
    pub struct Config32 {
        pub return_mode: ReturnMode,
        pub distance_resolution: Length,
        pub lasers: [Beam; 32],
    }

    // impls

    impl Config {
        pub fn try_into_kind(
            self,
        ) -> Result<FormatKind<Config16, Config32, Config16, Config32>, Self> {
            use FormatKind as K;
            use ReturnMode::*;

            let Self {
                return_mode,
                distance_resolution,
                lasers,
            } = self;

            Ok(match (return_mode, lasers.len()) {
                (Strongest | Last, 16) => K::from_s16(Config16 {
                    return_mode,
                    distance_resolution,
                    lasers: lasers.try_into().unwrap(),
                }),
                (Dual, 16) => K::from_d16(Config16 {
                    return_mode,
                    distance_resolution,
                    lasers: lasers.try_into().unwrap(),
                }),
                (Strongest | Last, 32) => K::from_s32(Config32 {
                    return_mode,
                    distance_resolution,
                    lasers: lasers.try_into().unwrap(),
                }),
                (Dual, 32) => K::from_d32(Config32 {
                    return_mode,
                    distance_resolution,
                    lasers: lasers.try_into().unwrap(),
                }),
                _ => {
                    return Err(Self {
                        return_mode,
                        distance_resolution,
                        lasers,
                    })
                }
            })
        }

        pub fn try_format(&self) -> Option<Format> {
            Format::try_new(self.lasers.len(), self.return_mode)
        }

        pub fn format(&self) -> Format {
            self.try_format().unwrap()
        }

        pub fn new_vlp_16_last() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_16();

            Self {
                return_mode: ReturnMode::Last,
                lasers,
                distance_resolution,
            }
        }

        pub fn new_vlp_16_strongest() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_16();

            Self {
                return_mode: ReturnMode::Strongest,
                lasers,
                distance_resolution,
            }
        }

        pub fn new_vlp_16_dual() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_16();

            Self {
                return_mode: ReturnMode::Dual,
                lasers,
                distance_resolution,
            }
        }

        pub fn new_puck_hires_last() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_hires();

            Self {
                return_mode: ReturnMode::Last,
                lasers,
                distance_resolution,
            }
        }

        pub fn new_puck_hires_strongest() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_hires();

            Self {
                return_mode: ReturnMode::Strongest,
                lasers,
                distance_resolution,
            }
        }

        pub fn new_puck_hires_dual() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_hires();

            Self {
                return_mode: ReturnMode::Dual,
                lasers,
                distance_resolution,
            }
        }

        pub fn new_puck_lite_last() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_lite();

            Self {
                return_mode: ReturnMode::Last,
                lasers,
                distance_resolution,
            }
        }

        pub fn new_puck_lite_strongest() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_lite();

            Self {
                return_mode: ReturnMode::Strongest,
                lasers,
                distance_resolution,
            }
        }

        pub fn new_puck_lite_dual() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_lite();

            Self {
                return_mode: ReturnMode::Dual,
                lasers,
                distance_resolution,
            }
        }

        pub fn new_vlp_32c_last() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_32c();

            Self {
                return_mode: ReturnMode::Last,
                lasers,
                distance_resolution,
            }
        }

        pub fn new_vlp_32c_strongest() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_32c();

            Self {
                return_mode: ReturnMode::Strongest,
                lasers,
                distance_resolution,
            }
        }

        pub fn new_vlp_32c_dual() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_32c();

            Self {
                return_mode: ReturnMode::Dual,
                lasers,
                distance_resolution,
            }
        }
    }

    impl Config16 {
        pub fn format(&self) -> Format {
            use Format::*;
            use ReturnMode::*;

            match self.return_mode {
                Strongest | Last => Single16,
                Dual => Dual16,
            }
        }

        pub fn new_vlp_16_last() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_16();

            Self {
                return_mode: ReturnMode::Last,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }

        pub fn new_vlp_16_strongest() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_16();

            Self {
                return_mode: ReturnMode::Strongest,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }

        pub fn new_vlp_16_dual() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_16();

            Self {
                return_mode: ReturnMode::Dual,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }

        pub fn new_puck_hires_last() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_hires();

            Self {
                return_mode: ReturnMode::Last,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }

        pub fn new_puck_hires_strongest() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_hires();

            Self {
                return_mode: ReturnMode::Strongest,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }

        pub fn new_puck_hires_dual() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_hires();

            Self {
                return_mode: ReturnMode::Dual,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }

        pub fn new_puck_lite_last() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_lite();

            Self {
                return_mode: ReturnMode::Last,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }

        pub fn new_puck_lite_strongest() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_lite();

            Self {
                return_mode: ReturnMode::Strongest,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }

        pub fn new_puck_lite_dual() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_puck_lite();

            Self {
                return_mode: ReturnMode::Dual,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }
    }

    impl Config32 {
        pub fn format(&self) -> Format {
            use Format::*;
            use ReturnMode::*;

            match self.return_mode {
                Strongest | Last => Single32,
                Dual => Dual32,
            }
        }

        pub fn new_vlp_32c_last() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_32c();

            Self {
                return_mode: ReturnMode::Last,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }

        pub fn new_vlp_32c_strongest() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_32c();

            Self {
                return_mode: ReturnMode::Strongest,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }

        pub fn new_vlp_32c_dual() -> Self {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = BeamConfig::new_vlp_32c();

            Self {
                return_mode: ReturnMode::Dual,
                lasers: lasers.try_into().unwrap(),
                distance_resolution,
            }
        }
    }

    impl From<Config16> for Config {
        fn from(from: Config16) -> Self {
            let Config16 {
                return_mode,
                lasers,
                distance_resolution,
            } = from;
            Self {
                return_mode,
                lasers: lasers.into(),
                distance_resolution,
            }
        }
    }

    impl From<Config32> for Config {
        fn from(from: Config32) -> Self {
            let Config32 {
                return_mode,
                lasers,
                distance_resolution,
            } = from;
            Self {
                return_mode,
                lasers: lasers.into(),
                distance_resolution,
            }
        }
    }

    impl TryFrom<Config> for Config16 {
        type Error = Config;

        fn try_from(from: Config) -> Result<Self, Self::Error> {
            let Config {
                return_mode,
                lasers,
                distance_resolution,
            } = from;

            let lasers = lasers.try_into().map_err(|lasers| Config {
                return_mode,
                lasers,
                distance_resolution,
            })?;

            Ok(Self {
                return_mode,
                lasers,
                distance_resolution,
            })
        }
    }

    impl TryFrom<Config> for Config32 {
        type Error = Config;

        fn try_from(from: Config) -> Result<Self, Self::Error> {
            let Config {
                return_mode,
                lasers,
                distance_resolution,
            } = from;

            let lasers = lasers.try_into().map_err(|lasers| Config {
                return_mode,
                lasers,
                distance_resolution,
            })?;

            Ok(Self {
                return_mode,
                lasers,
                distance_resolution,
            })
        }
    }
}

pub use params::*;
mod params {
    use super::*;

    #[derive(Debug, Clone)]
    pub(super) struct BeamConfig {
        pub lasers: Vec<Beam>,
        pub distance_resolution: Length,
    }

    #[derive(Debug, Clone)]
    pub(super) struct BeamConfig16 {
        pub lasers: [Beam; 16],
        pub distance_resolution: Length,
    }

    #[derive(Debug, Clone)]
    pub(super) struct BeamConfig32 {
        pub lasers: [Beam; 32],
        pub distance_resolution: Length,
    }

    #[derive(Debug, Clone)]
    pub struct Beam {
        pub elevation: Angle,
        pub azimuth_offset: Angle,
        pub vertical_offset: Length,
        pub horizontal_offset: Length,
    }

    impl BeamConfig {
        pub fn new_vlp_16() -> Self {
            let lasers: Vec<_> = izip!(
                consts::vlp_16::ELEVAION_DEGREES,
                consts::vlp_16::VERTICAL_OFFSETS,
                consts::vlp_16::HORIZONTAL_OFFSETS,
                consts::vlp_16::AZIMUTH_OFFSETS,
            )
            .map(
                |(elevation, vertical_offset, horizontal_offset, azimuth_offset)| Beam {
                    elevation: Angle::from_degrees(elevation),
                    vertical_offset: Length::from_millimeters(vertical_offset),
                    horizontal_offset: Length::from_millimeters(horizontal_offset),
                    azimuth_offset: Angle::from_degrees(azimuth_offset),
                },
            )
            .collect();

            Self {
                lasers,
                distance_resolution: *consts::vlp_16::DISTANCE_RESOLUTION,
            }
        }

        pub fn new_puck_hires() -> Self {
            let lasers: Vec<_> = izip!(
                consts::puck_hires::ELEVAION_DEGREES,
                consts::puck_hires::VERTICAL_OFFSETS,
                consts::puck_hires::HORIZONTAL_OFFSETS,
                consts::puck_hires::AZIMUTH_OFFSETS,
            )
            .map(
                |(elevation, vertical_offset, horizontal_offset, azimuth_offset)| Beam {
                    elevation: Angle::from_degrees(elevation),
                    vertical_offset: Length::from_millimeters(vertical_offset),
                    horizontal_offset: Length::from_millimeters(horizontal_offset),
                    azimuth_offset: Angle::from_degrees(azimuth_offset),
                },
            )
            .collect();

            Self {
                lasers,
                distance_resolution: *consts::puck_hires::DISTANCE_RESOLUTION,
            }
        }

        pub fn new_puck_lite() -> Self {
            let lasers: Vec<_> = izip!(
                consts::puck_lite::ELEVAION_DEGREES,
                consts::puck_lite::VERTICAL_OFFSETS,
                consts::puck_lite::HORIZONTAL_OFFSETS,
                consts::puck_lite::AZIMUTH_OFFSETS,
            )
            .map(
                |(elevation, vertical_offset, horizontal_offset, azimuth_offset)| Beam {
                    elevation: Angle::from_degrees(elevation),
                    vertical_offset: Length::from_millimeters(vertical_offset),
                    horizontal_offset: Length::from_millimeters(horizontal_offset),
                    azimuth_offset: Angle::from_degrees(azimuth_offset),
                },
            )
            .collect();

            Self {
                lasers,
                distance_resolution: *consts::puck_lite::DISTANCE_RESOLUTION,
            }
        }

        pub fn new_vlp_32c() -> Self {
            let lasers: Vec<_> = izip!(
                consts::vlp_32c::ELEVAION_DEGREES,
                consts::vlp_32c::VERTICAL_OFFSETS,
                consts::vlp_32c::HORIZONTAL_OFFSETS,
                consts::vlp_32c::AZIMUTH_OFFSETS,
            )
            .map(
                |(elevation, vertical_offset, horizontal_offset, azimuth_offset)| Beam {
                    elevation: Angle::from_degrees(elevation),
                    vertical_offset: Length::from_millimeters(vertical_offset),
                    horizontal_offset: Length::from_millimeters(horizontal_offset),
                    azimuth_offset: Angle::from_degrees(azimuth_offset),
                },
            )
            .collect();

            Self {
                lasers,
                distance_resolution: *consts::vlp_32c::DISTANCE_RESOLUTION,
            }
        }
    }

    impl From<BeamConfig16> for BeamConfig {
        fn from(from: BeamConfig16) -> Self {
            let BeamConfig16 {
                lasers,
                distance_resolution,
            } = from;
            Self {
                lasers: lasers.into(),
                distance_resolution,
            }
        }
    }

    impl From<BeamConfig32> for BeamConfig {
        fn from(from: BeamConfig32) -> Self {
            let BeamConfig32 {
                lasers,
                distance_resolution,
            } = from;
            Self {
                lasers: lasers.into(),
                distance_resolution,
            }
        }
    }

    impl TryFrom<BeamConfig> for BeamConfig16 {
        type Error = BeamConfig;

        fn try_from(from: BeamConfig) -> Result<Self, Self::Error> {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = from;

            let lasers = lasers.try_into().map_err(|lasers| BeamConfig {
                lasers,
                distance_resolution,
            })?;

            Ok(Self {
                lasers,
                distance_resolution,
            })
        }
    }

    impl TryFrom<BeamConfig> for BeamConfig32 {
        type Error = BeamConfig;

        fn try_from(from: BeamConfig) -> Result<Self, Self::Error> {
            let BeamConfig {
                lasers,
                distance_resolution,
            } = from;

            let lasers = lasers.try_into().map_err(|lasers| BeamConfig {
                lasers,
                distance_resolution,
            })?;

            Ok(Self {
                lasers,
                distance_resolution,
            })
        }
    }
}
