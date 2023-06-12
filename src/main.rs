mod device;
mod header;
mod packet;
mod payload;

use device::Device;
use packet::Packet;
use payload::Payload;
use std::net::{SocketAddr, UdpSocket};

fn main() {
    // MAC address of target device
    let device_mac_hex = "d073d5xxxxxx";
    let device_mac_dec = u64::from_str_radix(device_mac_hex, 16).unwrap();

    let payload = Some(Payload::set_colour(0x0, 0x0, 0xFFFF, 0xAC0D, 0));

    let packet = Packet::new(Some(device_mac_dec), 102, payload);

    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    // Enable broadcast option
    socket
        .set_broadcast(true)
        .expect("Failed to set broadcast option");

    // Destination address
    let broadcast_address = "192.168.1.255";
    let broadcast_port = 56700;
    let dest_addr: SocketAddr = format!("{}:{}", broadcast_address, broadcast_port)
        .parse()
        .expect("Failed to parse destination address");

    let packet = Vec::from(packet);

    // Send the packet to the destination
    socket
        .send_to(&packet, dest_addr)
        .expect("Failed to send packet");
}
