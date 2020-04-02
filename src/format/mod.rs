mod header;
mod packet;
mod frame;

pub(crate) fn take_source_str(i: &[u8]) -> nom::IResult<&[u8], String> {
    let (i, sstr) = nom::combinator::map_res(nom::bytes::streaming::take(260usize), std::str::from_utf8)(i)?;
    let ret = String::from(sstr.trim_end().trim_end_matches('\0'));
    Ok((i, ret))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Demo {
    header: header::DemoHeader,
    frames: Vec<frame::Frame>
}
