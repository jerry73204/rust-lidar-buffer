//! Packet data parsing and conversion for Velodyne LiDARs.
//!
//! # Example
//!
//! ```rust
//! # fn main() -> eyre::Result<()> {
//! use velodyne_lidar::{types::measurements::Measurement, Config};
//!
//! let config = Config::new_vlp_32c_strongest();
//! let frame_iter =
//!     velodyne_lidar::iter::frame_xyz_iter_from_file(config, "tests/velodyne_vlp32.pcap")?;
//!
//! for frame in frame_iter {
//!     let frame = frame?;
//!
//!     for firing in frame.firing_iter() {
//!         for point in firing.point_iter() {
//!             let point = point.as_single().unwrap();
//!             let Measurement {
//!                 distance,
//!                 intensity,
//!                 xyz: [x, y, z],
//!             } = point.measurement;
//!             print!("dist: {distance}\t");
//!             print!("int: {distance}\t");
//!             println!("xyz: {x} {y} {z}");
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```

pub mod batcher;
pub mod config;
pub mod consts;
mod convert;
pub mod iter;
pub mod packet;
#[cfg(feature = "parallel")]
pub mod par_iter;
pub mod traits;
pub mod types;
mod utils;

pub use config::*;
pub use packet::{DataPacket, Packet, PositionPacket, ProductID, ReturnMode};

pub mod prelude {
    pub use crate::traits::{AzimuthRange, FiringLike, PointField};
}
