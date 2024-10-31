//! Packet parallel iterator creation functions.

use crate::Packet;
use eyre::Result;
use pcap::{Capture, Device};
use rayon::{iter::IterBridge, prelude::*};
use std::path::Path;

/// Creates a packet iterator from [pcap::Capture].
pub fn from_capture<A>(
    capture: Capture<A>,
) -> Result<IterBridge<impl Iterator<Item = Result<Packet, pcap::Error>> + Send>, pcap::Error>
where
    A: pcap::Activated,
{
    Ok(crate::iter::packet_iter_from_capture(capture)?.par_bridge())
}

/// Creates a packet iterator by loading from a file.
pub fn from_file<P>(
    path: P,
) -> Result<IterBridge<impl Iterator<Item = Result<Packet, pcap::Error>> + Send>, pcap::Error>
where
    P: AsRef<Path>,
{
    let capture: Capture<pcap::Offline> = Capture::from_file(path)?;
    from_capture(capture)
}

/// Creates a packet iterator by reading a device.
pub fn from_device<D>(
    device: D,
) -> Result<IterBridge<impl Iterator<Item = Result<Packet, pcap::Error>> + Send>, pcap::Error>
where
    D: Into<Device>,
{
    let capture: Capture<pcap::Inactive> = Capture::from_device(device)?;
    let capture = capture.open()?;
    from_capture(capture)
}
