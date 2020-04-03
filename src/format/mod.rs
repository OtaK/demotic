mod header;
mod packet;
mod frame;

pub(crate) mod parsers {
    pub(crate) fn take_source_str(i: &[u8]) -> nom::IResult<&[u8], String> {
        let (i, sstr) = nom::combinator::map_res(nom::bytes::streaming::take(260usize), std::str::from_utf8)(i)?;
        debug!("Read raw string: {}", sstr);
        let ret = String::from(sstr.trim_end().trim_end_matches('\0'));
        debug!("Trimmed raw string: {}", ret);
        Ok((i, ret))
    }

    #[inline(always)]
    pub(crate) fn le_i32_as_u32(i: &[u8]) -> nom::IResult<&[u8], u32> {
        let (i, num) = nom::number::streaming::le_i32(i)?;
        debug!("Found LE i32: {} [as u32: {}]", num, num as u32);
        Ok((i, num as u32))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Demo {
    header: header::DemoHeader,
    frames: Vec<frame::Frame>
}
