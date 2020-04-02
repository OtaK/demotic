mod header;
mod packet;
mod frame;

pub(crate) mod parsers {
    pub(crate) fn take_source_str(i: &[u8]) -> nom::IResult<&[u8], String> {
        let (i, sstr) = nom::combinator::map_res(nom::bytes::streaming::take(260usize), std::str::from_utf8)(i)?;
        let ret = String::from(sstr.trim_end().trim_end_matches('\0'));
        Ok((i, ret))
    }

    pub(crate) fn le_i32_as_u16(i: &[u8]) -> nom::IResult<&[u8], u16> {
        nom::combinator::map(nom::number::streaming::le_i32, |i| i as u16)(i)
    }

    pub(crate) fn be_i32_as_u16(i: &[u8]) -> nom::IResult<&[u8], u16> {
        nom::combinator::map(nom::number::streaming::be_i32, |i| i as u16)(i)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Demo {
    header: header::DemoHeader,
    frames: Vec<frame::Frame>
}
