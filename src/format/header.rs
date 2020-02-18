use nom::{
    do_parse,
    tag,
    named,
    map,
    map_res,
    take_till,
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
    ticks: u32,
    frames: u32,
    signon_len: u32,
}

named!(till_eol, take_till!(|c| c == b'\n'));

impl crate::Parsable for DemoHeader {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self> {
        do_parse!(i,
            tag!(b"HL2DEMO\0")              >>
            demo_protocol: le_u32           >>
            network_protocol: le_u32        >>
            server_name: map!(map_res!(till_eol, std::str::from_utf8), String::from)    >>
            client_name: map!(map_res!(till_eol, std::str::from_utf8), String::from)    >>
            map_name: map!(map_res!(till_eol, std::str::from_utf8), String::from)       >>
            directory: map!(map_res!(till_eol, std::str::from_utf8), String::from)      >>
            playback_time: le_f32           >>
            ticks: le_u32                   >>
            frames: le_u32                  >>
            signon_len: le_u32              >>
            (DemoHeader {
                demo_protocol, network_protocol,
                server_name, client_name,
                map_name, directory,
                playback_time,
                ticks, frames,
                signon_len
            })
        )
    }
}
