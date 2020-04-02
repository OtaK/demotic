use crate::format::{
    packet::Packet,
    parsers::le_i32_as_u16,
};

use nom::{combinator::map, bytes::streaming::take, number::streaming::le_i32};

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    server_frame: u16,
    client_frame: u16,
    subpacket: Vec<u8>,
    packet: Packet
}

impl crate::Parsable for Frame {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self> {
        let (i, server_frame) = le_i32_as_u16(i)?;
        let (i, client_frame) = le_i32_as_u16(i)?;
        let (i, subpacket_len) = map(le_i32, |v| v as usize)(i)?;
        let (i, subpacket) = map(take(subpacket_len), Vec::from)(i)?;
        let (i, packet) = Packet::parse(i)?;

        Ok((i, Frame {
            server_frame,
            client_frame,
            subpacket,
            packet
        }))
    }
}

mod tests {

    #[test]
    fn can_parse_frame() {
        use crate::format::header::DemoHeader;
        use super::Frame;
        use crate::Parsable as _;
        use crate::{TEST_FILE_PATH, DemoReader};


        println!("Parsing: natus-vincere-vs-vitality-m2-dust2.dem");
        let reader = DemoReader::new(TEST_FILE_PATH).unwrap();
        let (reader, header) = DemoHeader::parse(&reader).unwrap();
        println!("Header {:#?}", header);

        let (_reader, frame) = Frame::parse(&reader).unwrap();
        println!("Frame: {:#?}", frame);
    }
}
