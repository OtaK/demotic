use crate::format::parsers::{be_i32_as_u16, le_i32_as_u16, take_source_str};
use nom::{
    do_parse,
    tag,
    map,
    number::streaming::le_f32
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ProtocolVersion {
    Legacy,
    Orange,
    Modern,
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self::Modern
    }
}

impl From<u16> for ProtocolVersion {
    fn from(v: u16) -> Self {
        match v {
            0..=13 => Self::Legacy,
            14..=35 => Self::Orange,
            _ => Self::Modern,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DemoHeader {
    demo_protocol: u16,
    network_protocol: u16,
    protocol_version: ProtocolVersion,
    server_name: String,
    client_name: String,
    map_name: String,
    directory: String,
    playback_time: std::time::Duration,
    tickrate: u16,
    ticks: u16,
    frames: u16,
    signon_len: u16,
}

impl crate::Parsable for DemoHeader {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self> {
        do_parse!(i,
            tag!(b"HL2DEMO\0")              >>
            demo_protocol: le_i32_as_u16    >>
            network_protocol: le_i32_as_u16 >>
            server_name: take_source_str    >>
            client_name: take_source_str    >>
            map_name: take_source_str       >>
            directory: take_source_str      >>
            playback_time: map!(le_f32, std::time::Duration::from_secs_f32) >>
            ticks: le_i32_as_u16            >>
            frames: le_i32_as_u16           >>
            signon_len: be_i32_as_u16       >>
            (DemoHeader {
                protocol_version: network_protocol.into(),
                demo_protocol, network_protocol,
                server_name, client_name,
                map_name, directory,
                tickrate: (ticks as f32 / playback_time.as_secs_f32()) as u16,
                playback_time,
                ticks, frames,
                signon_len
            })
        )
    }
}

mod tests {
    #[test]
    fn can_parse_header() {
        use super::DemoHeader;
        use crate::Parsable as _;
        use crate::{TEST_FILE_PATH, DemoReader};

        println!("Parsing: natus-vincere-vs-vitality-m2-dust2.dem");
        let reader = DemoReader::new(TEST_FILE_PATH).unwrap();
        let (_reader, header) = DemoHeader::parse(&reader).unwrap();

        println!("Header {:#?}", header);
    }
}
