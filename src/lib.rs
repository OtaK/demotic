mod format;
mod error;
pub trait Parsable: std::fmt::Debug + Clone + PartialEq {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
