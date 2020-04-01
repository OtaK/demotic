use crate::format::packet::Packet;

#[derive(Debug)]
pub struct Frame {
    server_frame: u16,
    client_frame: u16,
    subpacket: Vec<u8>,
    packet: Packet
}
