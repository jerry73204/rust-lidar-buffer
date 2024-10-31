use eyre::{ensure, Result};
use itertools::{izip, Itertools};
use pcap::Capture;
use velodyne_lidar::{config::Config, consts, DataPacket};

const UDP_HEADER_SIZE: usize = 42;

#[test]
fn velodyne_vlp_16_pcap_file() -> Result<()> {
    let data_packets: Vec<_> =
        velodyne_lidar::iter::packet_iter_from_file("test_files/velodyne_vlp16.pcap")?
            .map_ok(|packet| packet.try_into_data().ok())
            .flatten_ok()
            .try_collect()?;

    // timestamp test
    {
        let is_timestamp_valid = izip!(data_packets.iter(), data_packets.iter().skip(1))
            .all(|(former, latter)| former.toh < latter.toh);

        ensure!(is_timestamp_valid, "invalid timestamp detected");
    }

    // check if elevation angle is in order
    {
        let original = consts::vlp_16::ELEVAION_DEGREES;
        let mut sort = consts::vlp_16::ELEVAION_DEGREES;
        sort.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let idx_order = consts::vlp_16::ELEVAION_INDEX;

        for i in 0..idx_order.len() {
            assert!(sort[i] == original[idx_order[i]]);
        }
    }

    // convert to point cloud
    {
        let config = Config::new_vlp_16_strongest();
        velodyne_lidar::iter::data_packet_to_frame_xyz(config, data_packets.into_iter())?.count();
    }

    Ok(())
}

#[test]
fn velodyne_vlp_32_pcap_file() -> Result<()> {
    let mut cap = Capture::from_file("test_files/velodyne_vlp32.pcap")?;
    cap.filter("udp", true)?;

    let data_packets: Vec<_> = itertools::unfold(cap, |cap| {
        Some(loop {
            let packet = cap.next_packet().ok()?;
            let slice = &packet.data[UDP_HEADER_SIZE..];

            if let Ok(packet) = DataPacket::from_slice(slice) {
                break *packet;
            }
        })
    })
    .collect();

    // timestamp test
    {
        let is_timestamp_valid = izip!(data_packets.iter(), data_packets.iter().skip(1))
            .all(|(former, latter)| former.toh < latter.toh);

        ensure!(is_timestamp_valid, "invalid timestamp detected");
    }

    // check if elevation angle is in order
    {
        let original = consts::vlp_32c::ELEVAION_DEGREES;
        let mut sort = consts::vlp_32c::ELEVAION_DEGREES;
        sort.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let idx_order = consts::vlp_32c::ELEVAION_INDEX;

        for i in 0..idx_order.len() {
            assert!(sort[i] == original[idx_order[i]]);
        }
    }

    // convert to point cloud
    {
        let config = Config::new_vlp_32c_strongest();
        velodyne_lidar::iter::data_packet_to_frame_xyz(config, data_packets.into_iter())?.count();
    }

    Ok(())
}
