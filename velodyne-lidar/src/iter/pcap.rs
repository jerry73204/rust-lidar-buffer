//! Packet iterator creation functions.

use super::convert::{try_packet_to_frame_xyz, ResultFrameXyzIter};
use crate::{Config, Packet};
use eyre::Result;
use pcap::{Capture, Device};
use std::{iter, path::Path};

const UDP_HEADER_SIZE: usize = 42;

/// Creates a packet iterator from [pcap::Capture].
pub fn packet_iter_from_capture<A>(
    mut capture: Capture<A>,
) -> Result<impl Iterator<Item = Result<Packet, pcap::Error>> + Send, pcap::Error>
where
    A: pcap::Activated,
{
    capture.filter("udp", true)?;
    let iter = iter::from_fn(move || {
        Some(loop {
            let packet = match capture.next_packet() {
                Ok(packet) => packet,
                Err(pcap::Error::NoMorePackets) => return None,
                Err(err) => break Err(err),
            };
            let Some(slice) = packet.data.get(UDP_HEADER_SIZE..) else {
                continue;
            };
            let Ok(packet) = Packet::from_slice(slice) else {
                continue;
            };
            break Ok(packet);
        })
    });
    Ok(iter)
}

/// Creates a packet iterator by loading from a file.
pub fn packet_iter_from_file<P>(
    path: P,
) -> Result<impl Iterator<Item = Result<Packet, pcap::Error>> + Send, pcap::Error>
where
    P: AsRef<Path>,
{
    let capture: Capture<pcap::Offline> = Capture::from_file(path)?;
    packet_iter_from_capture(capture)
}

/// Creates a packet iterator by reading a device.
pub fn packet_iter_from_device<D>(
    device: D,
) -> Result<impl Iterator<Item = Result<Packet, pcap::Error>> + Send, pcap::Error>
where
    D: Into<Device>,
{
    let capture: Capture<pcap::Inactive> = Capture::from_device(device)?;
    let capture = capture.open()?;
    packet_iter_from_capture(capture)
}

pub fn frame_xyz_iter_from_capture<A>(
    config: Config,
    capture: Capture<A>,
) -> Result<ResultFrameXyzIter<'static, pcap::Error>>
where
    A: pcap::Activated + 'static,
{
    let packets = packet_iter_from_capture(capture)?;
    let iter = try_packet_to_frame_xyz(config, packets)?;
    Ok(iter)
}

pub fn frame_xyz_iter_from_file<P>(
    config: Config,
    path: P,
) -> Result<ResultFrameXyzIter<'static, pcap::Error>>
where
    P: AsRef<Path>,
{
    let capture: Capture<pcap::Offline> = Capture::from_file(path)?;
    frame_xyz_iter_from_capture(config, capture)
}

pub fn frame_xyz_iter_from_device<D>(
    config: Config,
    device: D,
) -> Result<ResultFrameXyzIter<'static, pcap::Error>>
where
    D: Into<Device>,
{
    let capture: Capture<pcap::Inactive> = Capture::from_device(device)?;
    let capture = capture.open()?;
    frame_xyz_iter_from_capture(config, capture)
}
