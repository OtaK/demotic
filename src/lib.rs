mod format;
mod error;

#[cfg(test)]
pub(crate) static TEST_FILE_PATH: &'static str = "assets/natus-vincere-vs-vitality-m2-dust2.dem";

pub(crate) type Buffer<'a> = &'a [u8];

pub trait Parsable: std::fmt::Debug + Clone + PartialEq {
    fn parse(i: Buffer) -> nom::IResult<Buffer, Self>;
}

#[derive(Debug)]
pub struct DemoReader {
    mmap: memmap::Mmap
}

impl DemoReader {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Result<Self, crate::error::DemoticError> {
        let file = std::fs::File::open(path)?;
        let mmap = unsafe { memmap::Mmap::map(&file)? };

        Ok(Self { mmap })
    }
}

impl std::ops::Deref for DemoReader {
    type Target = memmap::Mmap;
    fn deref(&self) -> &Self::Target {
        &self.mmap
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_open_mmaped_file() {
        use crate::{TEST_FILE_PATH, DemoReader};
        let _ = DemoReader::new(TEST_FILE_PATH).unwrap();
    }
}
