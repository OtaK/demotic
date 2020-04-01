use nom::{
    do_parse,
    tag,
    named,
    map,
    map_res,
    take_till,
    number::streaming::{le_u32, le_f32}
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum CommandTypeNP78 {
    SignOn = 1,
    Packet = 2,
    SyncTick = 3,
    ConsoleCommand = 4,
    UserCommand = 5,
    DataTables = 6,
    Stop = 7,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum CommandTypeNP1415 {
    SignOn = 1,
    Packet = 2,
    SyncTick = 3,
    ConsoleCommand = 4,
    UserCommand = 5,
    DataTables = 6,
    Stop = 7,
    StringTables = 8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum CommandTypeNP36 {
    SignOn = 1,
    Packet = 2,
    SyncTick = 3,
    ConsoleCommand = 4,
    UserCommand = 5,
    DataTables = 6,
    Stop = 7,
    CustomData = 8,
    StringTables = 9,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandType {
    Protocol7(CommandTypeNP78),
    Protocol14(CommandTypeNP1415),
    Protocol36(CommandTypeNP36),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    command_type: CommandType,
    tick_count: u8,
    packet_size: u8,
    data: Vec<u8>
}

/*impl crate::Parsable for Packet {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self> {
        do_parse!(i,
            command_type: le_u32           >>
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
}*/
