use crate::format::take_source_str;
use nom::{
    do_parse,
    tag,
    number::streaming::{le_u32, le_f32}
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DemoHeader {
    demo_protocol: u32,
    network_protocol: u32,
    server_name: String,
    client_name: String,
    map_name: String,
    directory: String,
    playback_time: f32,
    tickrate: u32,
    ticks: u32,
    frames: u32,
    signon_len: u32,
}

impl crate::Parsable for DemoHeader {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self> {
        do_parse!(i,
            tag!(b"HL2DEMO\0")              >>
            demo_protocol: le_u32           >>
            network_protocol: le_u32        >>
            server_name: take_source_str    >>
            client_name: take_source_str    >>
            map_name: take_source_str       >>
            directory: take_source_str      >>
            playback_time: le_f32           >>
            ticks: le_u32                   >>
            frames: le_u32                  >>
            signon_len: le_u32              >>
            (DemoHeader {
                demo_protocol, network_protocol,
                server_name, client_name,
                map_name, directory,
                tickrate: (ticks as f32 / playback_time) as u32,
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
        use std::io::BufRead;
        use super::DemoHeader;
        use crate::Parsable as _;

        println!("Parsing: natus-vincere-vs-vitality-m2-dust2.dem");
        let demo_file = std::fs::File::open("assets/natus-vincere-vs-vitality-m2-dust2.dem").unwrap();
        let mut reader = std::io::BufReader::new(demo_file);
        let mut buf = Vec::with_capacity(1024);
        reader.read_until(b'\n', &mut buf).unwrap();

        let (_, header) = DemoHeader::parse(&buf).unwrap();

        println!("Header {:#?}", header);
    }
}
