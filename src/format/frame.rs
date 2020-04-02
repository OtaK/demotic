use crate::format::packet::RawPacket;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    server_frame: u16,
    client_frame: u16,
    subpacket: Vec<u8>,
    packet: RawPacket
}
