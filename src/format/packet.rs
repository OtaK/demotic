#![allow(dead_code)]

use crate::Buffer;
use nom::{combinator::map, bytes::streaming::take};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum CommandTypeLegacy {
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
pub enum CommandTypeOrange {
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
pub enum CommandTypeModern {
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
pub enum CommandTypeActionType {
    Stop,
    ReadPacket,
    Ignore
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandType {
    Legacy(CommandTypeLegacy),
    Orange(CommandTypeOrange),
    Modern(CommandTypeModern),
}

impl CommandType {
    pub fn get_action_type(&self) -> CommandTypeActionType {
        match *self {
            Self::Legacy(s) => match s {
                CommandTypeLegacy::Stop => CommandTypeActionType::Stop,
                CommandTypeLegacy::SyncTick | CommandTypeLegacy::SignOn | CommandTypeLegacy::Packet => CommandTypeActionType::Ignore,
                _ => CommandTypeActionType::ReadPacket
            },
            Self::Orange(s) => match s {
                CommandTypeOrange::Stop => CommandTypeActionType::Stop,
                CommandTypeOrange::SyncTick | CommandTypeOrange::SignOn | CommandTypeOrange::Packet => CommandTypeActionType::Ignore,
                _ => CommandTypeActionType::ReadPacket
            },
            Self::Modern(s) => match s {
                CommandTypeModern::Stop => CommandTypeActionType::Stop,
                CommandTypeModern::SyncTick | CommandTypeModern::SignOn | CommandTypeModern::Packet => CommandTypeActionType::Ignore,
                _ => CommandTypeActionType::ReadPacket
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    //command_type: CommandType,
    //action_type: CommandTypeActionType,
    //tick_count: u16,
    data: Vec<u8>
}

impl crate::Parsable for Packet {
    fn parse(i: Buffer) -> nom::IResult<Buffer, Self> {
        let (i, len) = nom::number::streaming::le_i32(i)?;
        debug!("Packet len: {}", len);
        let (i, data) = map(take(len as usize), Into::into)(i)?;
        Ok((i, Packet { data }))
    }
    //action_type: command_type.get_action_type(),
                //command_type,
                //tick_count,
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_parse_packet() {
        let _ = pretty_env_logger::try_init();

        use crate::format::{header::DemoHeader, frame::Frame};
        use super::Packet;
        use crate::Parsable as _;
        use crate::{TEST_FILE_PATH, DemoReader};

        println!("Parsing: natus-vincere-vs-vitality-m2-dust2.dem");
        let reader = DemoReader::new(TEST_FILE_PATH).unwrap();
        let (reader, header) = DemoHeader::parse(&reader).unwrap();
        println!("Header {:#?}", header);

        let (reader, frame) = Frame::parse(&reader).unwrap();
        println!("Frame: {:#?}", frame);

        let (_reader, packet) = Packet::parse(&reader).unwrap();
        println!("Packet: {:#?}", packet);
    }
}
